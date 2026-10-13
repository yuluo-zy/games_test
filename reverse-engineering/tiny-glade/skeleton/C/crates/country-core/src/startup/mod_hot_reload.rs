// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/mod_hot_reload.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 182d03d22d19e66670353d30a28dc348f5de95b2a625af8f3fb6aad959bad414
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod_hot_reload.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 508, 534, 565, 1643, 1651, 1656, 1658, 1660, 1682, 1687, 1822
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use async_lock; // namespace candidate
//   package identities: [{"name":"async-lock","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.2"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use clutter_pack_pipe; // namespace candidate
//   package identities: [{"name":"clutter-pack-pipe","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71B4
// 首条位置: {"module":508,"inlinee":29108}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":508,"record_offset":128696,"parent_record_offset":128316,"ancestor_record_offset":128316,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":128344,"parent_record_offset":128316,"ancestor_record_offset":128316,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":508,"record_offset":128724,"parent_record_offset":128696,"ancestor_record_offset":128316,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":508,"record_offset":128516,"parent_record_offset":128368,"ancestor_record_offset":128316,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":508,"record_offset":128628,"parent_record_offset":128600,"ancestor_record_offset":128316,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":508,"record_offset":128600,"parent_record_offset":128368,"ancestor_record_offset":128316,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":508,"record_offset":128456,"parent_record_offset":128432,"ancestor_record_offset":128316,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":128540,"parent_record_offset":128516,"ancestor_record_offset":128316,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":128564,"parent_record_offset":128540,"ancestor_record_offset":128316,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":128432,"parent_record_offset":128368,"ancestor_record_offset":128316,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":128408,"parent_record_offset":128368,"ancestor_record_offset":128316,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":128368,"parent_record_offset":128344,"ancestor_record_offset":128316,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::impl$9::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14114
// 首条位置: {"module":1660,"inlinee":82196}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1660,"record_offset":38232,"parent_record_offset":37784,"ancestor_record_offset":37784,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$65::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":37812,"parent_record_offset":37784,"ancestor_record_offset":37784,"inlinee":20210,"source_line":2257,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":38100,"parent_record_offset":38076,"ancestor_record_offset":37784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":38148,"parent_record_offset":38124,"ancestor_record_offset":37784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1660,"record_offset":38124,"parent_record_offset":38100,"ancestor_record_offset":37784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":38076,"parent_record_offset":38052,"ancestor_record_offset":37784,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":38052,"parent_record_offset":38028,"ancestor_record_offset":37784,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1660,"record_offset":38028,"parent_record_offset":38004,"ancestor_record_offset":37784,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1660,"record_offset":38004,"parent_record_offset":37980,"ancestor_record_offset":37784,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":38352,"parent_record_offset":38328,"ancestor_record_offset":37784,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":38256,"parent_record_offset":38232,"ancestor_record_offset":37784,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":38304,"parent_record_offset":38280,"ancestor_record_offset":37784,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":38328,"parent_record_offset":38304,"ancestor_record_offset":37784,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":37908,"parent_record_offset":37884,"ancestor_record_offset":37784,"inlinee":11100,"source_line":1710,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":37956,"parent_record_offset":37932,"ancestor_record_offset":37784,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1660,"record_offset":37932,"parent_record_offset":37908,"ancestor_record_offset":37784,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":38280,"parent_record_offset":38256,"ancestor_record_offset":37784,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":37884,"parent_record_offset":37860,"ancestor_record_offset":37784,"inlinee":11098,"source_line":2383,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::PathBuf::as_path
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":37836,"parent_record_offset":37812,"ancestor_record_offset":37784,"inlinee":11095,"source_line":1264,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1660,"record_offset":37860,"parent_record_offset":37836,"ancestor_record_offset":37784,"inlinee":11097,"source_line":2066,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1660,"record_offset":37980,"parent_record_offset":37956,"ancestor_record_offset":37784,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71B6
// 首条位置: {"module":508,"inlinee":29110}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":129120,"parent_record_offset":129092,"ancestor_record_offset":129092,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":508,"record_offset":129296,"parent_record_offset":129144,"ancestor_record_offset":129092,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":508,"record_offset":129408,"parent_record_offset":129380,"ancestor_record_offset":129092,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":508,"record_offset":129380,"parent_record_offset":129144,"ancestor_record_offset":129092,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":508,"record_offset":129236,"parent_record_offset":129212,"ancestor_record_offset":129092,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":129320,"parent_record_offset":129296,"ancestor_record_offset":129092,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":129344,"parent_record_offset":129320,"ancestor_record_offset":129092,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":129212,"parent_record_offset":129144,"ancestor_record_offset":129092,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":129184,"parent_record_offset":129144,"ancestor_record_offset":129092,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":129144,"parent_record_offset":129120,"ancestor_record_offset":129092,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::watchable_mod_roots
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AD5
// 首条位置: {"module":565,"inlinee":35541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":46544,"parent_record_offset":46472,"ancestor_record_offset":46472,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":46716,"parent_record_offset":46472,"ancestor_record_offset":46472,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":46660,"parent_record_offset":46472,"ancestor_record_offset":46472,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":48320,"parent_record_offset":46472,"ancestor_record_offset":46472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49424,"parent_record_offset":49400,"ancestor_record_offset":46472,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49448,"parent_record_offset":49424,"ancestor_record_offset":46472,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49472,"parent_record_offset":49448,"ancestor_record_offset":46472,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49400,"parent_record_offset":49344,"ancestor_record_offset":46472,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":46472,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49344,"parent_record_offset":49320,"ancestor_record_offset":46472,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49320,"parent_record_offset":49296,"ancestor_record_offset":46472,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":48232,"parent_record_offset":48208,"ancestor_record_offset":46472,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":565,"record_offset":48284,"parent_record_offset":48232,"ancestor_record_offset":46472,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":47868,"parent_record_offset":47844,"ancestor_record_offset":46472,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":47320,"parent_record_offset":47292,"ancestor_record_offset":46472,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":47220,"parent_record_offset":47016,"ancestor_record_offset":46472,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":565,"record_offset":48640,"parent_record_offset":48616,"ancestor_record_offset":46472,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":47700,"parent_record_offset":47668,"ancestor_record_offset":46472,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":46684,"parent_record_offset":46660,"ancestor_record_offset":46472,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":47760,"parent_record_offset":47736,"ancestor_record_offset":46472,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":47820,"parent_record_offset":47560,"ancestor_record_offset":46472,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":565,"record_offset":47612,"parent_record_offset":47560,"ancestor_record_offset":46472,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":47736,"parent_record_offset":47560,"ancestor_record_offset":46472,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":565,"record_offset":46572,"parent_record_offset":46544,"ancestor_record_offset":46472,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":565,"record_offset":46596,"parent_record_offset":46572,"ancestor_record_offset":46472,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":47184,"parent_record_offset":47160,"ancestor_record_offset":46472,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":49652,"parent_record_offset":49624,"ancestor_record_offset":46472,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":49680,"parent_record_offset":49652,"ancestor_record_offset":46472,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":47348,"parent_record_offset":47292,"ancestor_record_offset":46472,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":48860,"parent_record_offset":48836,"ancestor_record_offset":46472,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":46928,"parent_record_offset":46892,"ancestor_record_offset":46472,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":47048,"parent_record_offset":47016,"ancestor_record_offset":46472,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":48672,"parent_record_offset":48588,"ancestor_record_offset":46472,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":47940,"parent_record_offset":47916,"ancestor_record_offset":46472,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":47988,"parent_record_offset":47560,"ancestor_record_offset":46472,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":46628,"parent_record_offset":46544,"ancestor_record_offset":46472,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":49128,"parent_record_offset":49104,"ancestor_record_offset":46472,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":47916,"parent_record_offset":47892,"ancestor_record_offset":46472,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":47136,"parent_record_offset":47016,"ancestor_record_offset":46472,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":48980,"parent_record_offset":48956,"ancestor_record_offset":46472,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":47892,"parent_record_offset":47868,"ancestor_record_offset":46472,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":47160,"parent_record_offset":47136,"ancestor_record_offset":46472,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48804,"parent_record_offset":48776,"ancestor_record_offset":46472,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48836,"parent_record_offset":48804,"ancestor_record_offset":46472,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48776,"parent_record_offset":48736,"ancestor_record_offset":46472,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":565,"record_offset":48616,"parent_record_offset":48588,"ancestor_record_offset":46472,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":47640,"parent_record_offset":47560,"ancestor_record_offset":46472,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":47560,"parent_record_offset":47524,"ancestor_record_offset":46472,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49104,"parent_record_offset":48500,"ancestor_record_offset":46472,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48956,"parent_record_offset":48736,"ancestor_record_offset":46472,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48588,"parent_record_offset":48532,"ancestor_record_offset":46472,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":46472,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":47668,"parent_record_offset":47640,"ancestor_record_offset":46472,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49592,"parent_record_offset":49560,"ancestor_record_offset":46472,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48560,"parent_record_offset":48532,"ancestor_record_offset":46472,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48500,"parent_record_offset":48440,"ancestor_record_offset":46472,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48440,"parent_record_offset":48416,"ancestor_record_offset":46472,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49560,"parent_record_offset":48440,"ancestor_record_offset":46472,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48472,"parent_record_offset":48440,"ancestor_record_offset":46472,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48532,"parent_record_offset":48500,"ancestor_record_offset":46472,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49624,"parent_record_offset":49592,"ancestor_record_offset":46472,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48416,"parent_record_offset":48392,"ancestor_record_offset":46472,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48708,"parent_record_offset":48500,"ancestor_record_offset":46472,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":47524,"parent_record_offset":47488,"ancestor_record_offset":46472,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":47460,"parent_record_offset":46752,"ancestor_record_offset":46472,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":47488,"parent_record_offset":46752,"ancestor_record_offset":46472,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":46752,"parent_record_offset":46716,"ancestor_record_offset":46472,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":46836,"parent_record_offset":46808,"ancestor_record_offset":46472,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":47096,"parent_record_offset":47072,"ancestor_record_offset":46472,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":46956,"parent_record_offset":46928,"ancestor_record_offset":46472,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":47292,"parent_record_offset":46892,"ancestor_record_offset":46472,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":46808,"parent_record_offset":46752,"ancestor_record_offset":46472,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":46984,"parent_record_offset":46956,"ancestor_record_offset":46472,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":47016,"parent_record_offset":46984,"ancestor_record_offset":46472,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":46892,"parent_record_offset":46864,"ancestor_record_offset":46472,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":46864,"parent_record_offset":46836,"ancestor_record_offset":46472,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::watchable_mod_roots::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1591F
// 首条位置: {"module":1822,"inlinee":88351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":16988,"parent_record_offset":16960,"ancestor_record_offset":16960,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":16960,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":16960,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":17236,"parent_record_offset":17212,"ancestor_record_offset":16960,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":17164,"parent_record_offset":17108,"ancestor_record_offset":16960,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":17136,"parent_record_offset":17108,"ancestor_record_offset":16960,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":16960,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":16960,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":16960,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::watchable_mod_roots::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1592B
// 首条位置: {"module":1822,"inlinee":88363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1822,"record_offset":46800,"parent_record_offset":45776,"ancestor_record_offset":45776,"inlinee":6131,"source_line":36,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1822,"record_offset":46772,"parent_record_offset":45776,"ancestor_record_offset":45776,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":47232,"parent_record_offset":45776,"ancestor_record_offset":45776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1822,"record_offset":45828,"parent_record_offset":45776,"ancestor_record_offset":45776,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":47104,"parent_record_offset":47080,"ancestor_record_offset":45776,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":47128,"parent_record_offset":47104,"ancestor_record_offset":45776,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":47152,"parent_record_offset":47128,"ancestor_record_offset":45776,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":47080,"parent_record_offset":47024,"ancestor_record_offset":45776,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":47052,"parent_record_offset":47024,"ancestor_record_offset":45776,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":47024,"parent_record_offset":47000,"ancestor_record_offset":45776,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":47000,"parent_record_offset":46976,"ancestor_record_offset":45776,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1822,"record_offset":46120,"parent_record_offset":46096,"ancestor_record_offset":45776,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1822,"record_offset":46204,"parent_record_offset":46180,"ancestor_record_offset":45776,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1822,"record_offset":46596,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1822,"record_offset":46352,"parent_record_offset":46324,"ancestor_record_offset":45776,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1822,"record_offset":46324,"parent_record_offset":46296,"ancestor_record_offset":45776,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":46144,"parent_record_offset":46120,"ancestor_record_offset":45776,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":46832,"parent_record_offset":46800,"ancestor_record_offset":45776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1822,"record_offset":46472,"parent_record_offset":46448,"ancestor_record_offset":45776,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":46264,"parent_record_offset":46236,"ancestor_record_offset":45776,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":46296,"parent_record_offset":46264,"ancestor_record_offset":45776,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":46236,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":46096,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":46624,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":46180,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1822,"record_offset":46040,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":45876,"parent_record_offset":45852,"ancestor_record_offset":45776,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":46564,"parent_record_offset":46396,"ancestor_record_offset":45776,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":45908,"parent_record_offset":45876,"ancestor_record_offset":45776,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46504,"parent_record_offset":46420,"ancestor_record_offset":45776,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46448,"parent_record_offset":46420,"ancestor_record_offset":45776,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46716,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46420,"parent_record_offset":46396,"ancestor_record_offset":45776,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":45960,"parent_record_offset":45936,"ancestor_record_offset":45776,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":45936,"parent_record_offset":45876,"ancestor_record_offset":45776,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":45984,"parent_record_offset":45960,"ancestor_record_offset":45776,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46068,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":46396,"parent_record_offset":45984,"ancestor_record_offset":45776,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1822,"record_offset":45852,"parent_record_offset":45828,"ancestor_record_offset":45776,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches
// 记录形式: compiled_procedure; instances=1
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":46236,"rva":11733392,"va":5380442512}
// 编译实例 RVAs: 0xb30990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4202272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8201f4000","size":5,"soft_fail":false,"target_va":5384644912,"va":5380442635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36170528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e820eb2702","size":5,"soft_fail":false,"target_va":5416616112,"va":5380445579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36172950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e896f42702","size":5,"soft_fail":false,"target_va":5416616304,"va":5380443349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4165774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e88e903f00","size":5,"soft_fail":false,"target_va":5384609408,"va":5380443629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4765737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8d747b7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380444420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-4764373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e82b4db7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380443072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11722214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e81a224dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380444401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11206190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8d20155ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5380446633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11181679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8916155ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5380446794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11104902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e87a8d56ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5380443745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140927690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2135464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8586adfff","size":5,"soft_fail":false,"target_va":5378307728,"va":5380443187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d510
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1852671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e801bbe3ff","size":5,"soft_fail":false,"target_va":5378594064,"va":5380446730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1787944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8d8b7e4ff","size":5,"soft_fail":false,"target_va":5378657872,"va":5380445811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1644190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e862e9e6ff","size":5,"soft_fail":false,"target_va":5378801568,"va":5380445753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a9d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1604526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e85284e7ff","size":5,"soft_fail":false,"target_va":5378841952,"va":5380446473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a36340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1026500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e83c56f0ff","size":5,"soft_fail":false,"target_va":5379416896,"va":5380443391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a36850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1026388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8ac56f0ff","size":5,"soft_fail":false,"target_va":5379418192,"va":5380444575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a42610
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-979256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8c80ef1ff","size":5,"soft_fail":false,"target_va":5379466768,"va":5380446019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1443968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e880081600","size":5,"soft_fail":false,"target_va":5381890080,"va":5380446107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c939c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1452657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8712a1600","size":5,"soft_fail":false,"target_va":5381896640,"va":5380443978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162acb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11507683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8e397af00","size":5,"soft_fail":false,"target_va":5391953072,"va":5380445384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14170a4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12425622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e89699bd00","size":5,"soft_fail":false,"target_va":5392868544,"va":5380442917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b23a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13113086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8fe16c800","size":5,"soft_fail":false,"target_va":5393556384,"va":5380443293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c35ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17846841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e839521001","size":5,"soft_fail":false,"target_va":5398289632,"va":5380442786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35907515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8bbe72302","size":5,"soft_fail":false,"target_va":5416353040,"va":5380445520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d905d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36040872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8a8f02502","size":5,"soft_fail":false,"target_va":5416486352,"va":5380445475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36612605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8fda92e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380444446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36175596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8ecfe2702","size":5,"soft_fail":false,"target_va":5416620794,"va":5380445193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36175959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e857002802","size":5,"soft_fail":false,"target_va":5416621632,"va":5380445668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36174094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e80ef92702","size":5,"soft_fail":false,"target_va":5416619696,"va":5380445597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::canonicalize
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4278907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e87b4a4100","size":5,"soft_fail":false,"target_va":5384722976,"va":5380444064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4281278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8be534100","size":5,"soft_fail":false,"target_va":5384726688,"va":5380445405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::active_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2441540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e844412500","size":5,"soft_fail":false,"target_va":5382884112,"va":5380442567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2461046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380442512,"raw_hex":"e8768d2500","size":5,"soft_fail":false,"target_va":5382903888,"va":5380442837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":53596,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":50100,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":51144,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":53088,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$3::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":565,"record_offset":50032,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":35544,"source_line":15,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":565,"record_offset":51424,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":11390,"source_line":861,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":51256,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":49908,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":49940,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":50000,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":50736,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":565,"record_offset":50584,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":53200,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::mod_hot_reload::watchable_mod_roots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":46472,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":35541,"source_line":120,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":51312,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":51172,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":50680,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":27433,"source_line":479,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":53288,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":8114,"source_line":3307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":53684,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":50128,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":50624,"parent_record_offset":46236,"ancestor_record_offset":46236,"inlinee":12499,"source_line":735,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":54184,"parent_record_offset":54160,"ancestor_record_offset":46236,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":54208,"parent_record_offset":54184,"ancestor_record_offset":46236,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49424,"parent_record_offset":49400,"ancestor_record_offset":46236,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49448,"parent_record_offset":49424,"ancestor_record_offset":46236,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":54232,"parent_record_offset":54208,"ancestor_record_offset":46236,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49472,"parent_record_offset":49448,"ancestor_record_offset":46236,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":54160,"parent_record_offset":54012,"ancestor_record_offset":46236,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":49400,"parent_record_offset":49344,"ancestor_record_offset":46236,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":51544,"parent_record_offset":51516,"ancestor_record_offset":46236,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":53952,"parent_record_offset":53924,"ancestor_record_offset":46236,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":51628,"parent_record_offset":51604,"ancestor_record_offset":46236,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":46236,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49344,"parent_record_offset":49320,"ancestor_record_offset":46236,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":51604,"parent_record_offset":51572,"ancestor_record_offset":46236,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":51572,"parent_record_offset":51544,"ancestor_record_offset":46236,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":54012,"parent_record_offset":53980,"ancestor_record_offset":46236,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":53980,"parent_record_offset":53952,"ancestor_record_offset":46236,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":49320,"parent_record_offset":49296,"ancestor_record_offset":46236,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":54048,"parent_record_offset":54012,"ancestor_record_offset":46236,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":565,"record_offset":53888,"parent_record_offset":53824,"ancestor_record_offset":46236,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":55804,"parent_record_offset":55768,"ancestor_record_offset":46236,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":55248,"parent_record_offset":55224,"ancestor_record_offset":46236,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":53860,"parent_record_offset":53824,"ancestor_record_offset":46236,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":51516,"parent_record_offset":51480,"ancestor_record_offset":46236,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":53924,"parent_record_offset":53888,"ancestor_record_offset":46236,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":53796,"parent_record_offset":53768,"ancestor_record_offset":46236,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":48232,"parent_record_offset":48208,"ancestor_record_offset":46236,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":46544,"parent_record_offset":46472,"ancestor_record_offset":46236,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":52332,"parent_record_offset":52308,"ancestor_record_offset":46236,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":53824,"parent_record_offset":53796,"ancestor_record_offset":46236,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":565,"record_offset":48284,"parent_record_offset":48232,"ancestor_record_offset":46236,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":47868,"parent_record_offset":47844,"ancestor_record_offset":46236,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":565,"record_offset":53768,"parent_record_offset":53740,"ancestor_record_offset":46236,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":565,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":46236,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":565,"record_offset":51480,"parent_record_offset":51452,"ancestor_record_offset":46236,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":565,"record_offset":54072,"parent_record_offset":54048,"ancestor_record_offset":46236,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":47320,"parent_record_offset":47292,"ancestor_record_offset":46236,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":47220,"parent_record_offset":47016,"ancestor_record_offset":46236,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":50648,"parent_record_offset":50624,"ancestor_record_offset":46236,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":565,"record_offset":48640,"parent_record_offset":48616,"ancestor_record_offset":46236,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":47700,"parent_record_offset":47668,"ancestor_record_offset":46236,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":46684,"parent_record_offset":46660,"ancestor_record_offset":46236,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":47760,"parent_record_offset":47736,"ancestor_record_offset":46236,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":47820,"parent_record_offset":47560,"ancestor_record_offset":46236,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":565,"record_offset":47612,"parent_record_offset":47560,"ancestor_record_offset":46236,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":46716,"parent_record_offset":46472,"ancestor_record_offset":46236,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":47736,"parent_record_offset":47560,"ancestor_record_offset":46236,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":46660,"parent_record_offset":46472,"ancestor_record_offset":46236,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":565,"record_offset":46572,"parent_record_offset":46544,"ancestor_record_offset":46236,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":565,"record_offset":46596,"parent_record_offset":46572,"ancestor_record_offset":46236,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":47184,"parent_record_offset":47160,"ancestor_record_offset":46236,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":49652,"parent_record_offset":49624,"ancestor_record_offset":46236,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":49680,"parent_record_offset":49652,"ancestor_record_offset":46236,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":52928,"parent_record_offset":52900,"ancestor_record_offset":46236,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":47348,"parent_record_offset":47292,"ancestor_record_offset":46236,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":48860,"parent_record_offset":48836,"ancestor_record_offset":46236,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":46928,"parent_record_offset":46892,"ancestor_record_offset":46236,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":51280,"parent_record_offset":51256,"ancestor_record_offset":46236,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":47048,"parent_record_offset":47016,"ancestor_record_offset":46236,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":48672,"parent_record_offset":48588,"ancestor_record_offset":46236,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":54288,"parent_record_offset":53888,"ancestor_record_offset":46236,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":47940,"parent_record_offset":47916,"ancestor_record_offset":46236,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":54312,"parent_record_offset":54288,"ancestor_record_offset":46236,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":47988,"parent_record_offset":47560,"ancestor_record_offset":46236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":46628,"parent_record_offset":46544,"ancestor_record_offset":46236,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":49128,"parent_record_offset":49104,"ancestor_record_offset":46236,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":47916,"parent_record_offset":47892,"ancestor_record_offset":46236,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":47136,"parent_record_offset":47016,"ancestor_record_offset":46236,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":48980,"parent_record_offset":48956,"ancestor_record_offset":46236,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":55836,"parent_record_offset":55804,"ancestor_record_offset":46236,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":47892,"parent_record_offset":47868,"ancestor_record_offset":46236,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":53240,"parent_record_offset":53200,"ancestor_record_offset":46236,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":47160,"parent_record_offset":47136,"ancestor_record_offset":46236,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":50060,"parent_record_offset":50032,"ancestor_record_offset":46236,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":53112,"parent_record_offset":53088,"ancestor_record_offset":46236,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":53136,"parent_record_offset":53112,"ancestor_record_offset":46236,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50504,"parent_record_offset":50476,"ancestor_record_offset":46236,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50988,"parent_record_offset":50800,"ancestor_record_offset":46236,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50196,"parent_record_offset":50164,"ancestor_record_offset":46236,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":51076,"parent_record_offset":51048,"ancestor_record_offset":46236,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":53624,"parent_record_offset":53596,"ancestor_record_offset":46236,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":55284,"parent_record_offset":55248,"ancestor_record_offset":46236,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50408,"parent_record_offset":50376,"ancestor_record_offset":46236,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":46236,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50220,"parent_record_offset":50196,"ancestor_record_offset":46236,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":50432,"parent_record_offset":50408,"ancestor_record_offset":46236,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":51012,"parent_record_offset":50988,"ancestor_record_offset":46236,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":55308,"parent_record_offset":55284,"ancestor_record_offset":46236,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":51100,"parent_record_offset":51076,"ancestor_record_offset":46236,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":55340,"parent_record_offset":55248,"ancestor_record_offset":46236,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":49968,"parent_record_offset":49940,"ancestor_record_offset":46236,"inlinee":35542,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48804,"parent_record_offset":48776,"ancestor_record_offset":46236,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48836,"parent_record_offset":48804,"ancestor_record_offset":46236,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":48776,"parent_record_offset":48736,"ancestor_record_offset":46236,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":565,"record_offset":48616,"parent_record_offset":48588,"ancestor_record_offset":46236,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":47640,"parent_record_offset":47560,"ancestor_record_offset":46236,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":47560,"parent_record_offset":47524,"ancestor_record_offset":46236,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49104,"parent_record_offset":48500,"ancestor_record_offset":46236,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48956,"parent_record_offset":48736,"ancestor_record_offset":46236,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48588,"parent_record_offset":48532,"ancestor_record_offset":46236,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":46236,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":47668,"parent_record_offset":47640,"ancestor_record_offset":46236,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49592,"parent_record_offset":49560,"ancestor_record_offset":46236,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48560,"parent_record_offset":48532,"ancestor_record_offset":46236,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48500,"parent_record_offset":48440,"ancestor_record_offset":46236,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48440,"parent_record_offset":48416,"ancestor_record_offset":46236,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49560,"parent_record_offset":48440,"ancestor_record_offset":46236,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48472,"parent_record_offset":48440,"ancestor_record_offset":46236,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48532,"parent_record_offset":48500,"ancestor_record_offset":46236,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":49624,"parent_record_offset":49592,"ancestor_record_offset":46236,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48416,"parent_record_offset":48392,"ancestor_record_offset":46236,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":48708,"parent_record_offset":48500,"ancestor_record_offset":46236,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":50704,"parent_record_offset":50680,"ancestor_record_offset":46236,"inlinee":27434,"source_line":368,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":47524,"parent_record_offset":47488,"ancestor_record_offset":46236,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":51360,"parent_record_offset":51336,"ancestor_record_offset":46236,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":51336,"parent_record_offset":51312,"ancestor_record_offset":46236,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":51384,"parent_record_offset":51360,"ancestor_record_offset":46236,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":47460,"parent_record_offset":46752,"ancestor_record_offset":46236,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":47488,"parent_record_offset":46752,"ancestor_record_offset":46236,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":46752,"parent_record_offset":46716,"ancestor_record_offset":46236,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":565,"record_offset":53712,"parent_record_offset":53684,"ancestor_record_offset":46236,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":565,"record_offset":53312,"parent_record_offset":53288,"ancestor_record_offset":46236,"inlinee":8115,"source_line":3035,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":46836,"parent_record_offset":46808,"ancestor_record_offset":46236,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":47096,"parent_record_offset":47072,"ancestor_record_offset":46236,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":46956,"parent_record_offset":46928,"ancestor_record_offset":46236,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":47292,"parent_record_offset":46892,"ancestor_record_offset":46236,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":46808,"parent_record_offset":46752,"ancestor_record_offset":46236,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":565,"record_offset":50376,"parent_record_offset":50348,"ancestor_record_offset":46236,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":565,"record_offset":50348,"parent_record_offset":50320,"ancestor_record_offset":46236,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":50800,"parent_record_offset":50764,"ancestor_record_offset":46236,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":50476,"parent_record_offset":50284,"ancestor_record_offset":46236,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":50284,"parent_record_offset":50256,"ancestor_record_offset":46236,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":50256,"parent_record_offset":50128,"ancestor_record_offset":46236,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":50764,"parent_record_offset":50736,"ancestor_record_offset":46236,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":565,"record_offset":53740,"parent_record_offset":53712,"ancestor_record_offset":46236,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":565,"record_offset":50164,"parent_record_offset":50128,"ancestor_record_offset":46236,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":565,"record_offset":51048,"parent_record_offset":50764,"ancestor_record_offset":46236,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":46984,"parent_record_offset":46956,"ancestor_record_offset":46236,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":47016,"parent_record_offset":46984,"ancestor_record_offset":46236,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":565,"record_offset":50320,"parent_record_offset":50284,"ancestor_record_offset":46236,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":46892,"parent_record_offset":46864,"ancestor_record_offset":46236,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":46864,"parent_record_offset":46836,"ancestor_record_offset":46236,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AD6
// 首条位置: {"module":565,"inlinee":35542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x140FA
// 首条位置: {"module":1658,"inlinee":82170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":534,"record_offset":40168,"rva":10694768,"va":5379403888}
// 编译实例 RVAs: 0xa33070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_strip_prefix
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5250384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8501d5000","size":5,"soft_fail":false,"target_va":5384654544,"va":5379404155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t37211084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8cccb3702","size":5,"soft_fail":false,"target_va":5416616112,"va":5379405023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t37211295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e89fcc3702","size":5,"soft_fail":false,"target_va":5416616304,"va":5379405004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3725584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8f026c7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379404267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-3725507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e83d27c7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379404206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10682060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e834015dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379404247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10165062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8bae464ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5379405505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10140691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8ed4365ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5379405806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008d710
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10116935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8b9a065ff","size":5,"soft_fail":false,"target_va":5369288464,"va":5379405394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14146309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e805dbd700","size":5,"soft_fail":false,"target_va":5393551264,"va":5379404950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37652168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8c8863e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379404883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37215695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8cfdd3702","size":5,"soft_fail":false,"target_va":5416620794,"va":5379405094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37216324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e844e03702","size":5,"soft_fail":false,"target_va":5416621632,"va":5379405303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5425900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8ecca5200","size":5,"soft_fail":false,"target_va":5384830080,"va":5379404175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::mod_hot_reload::classify
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1042108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8bce60f00","size":5,"soft_fail":false,"target_va":5380446832,"va":5379404719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::mod_hot_reload::event_paths
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1020329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8a9910f00","size":5,"soft_fail":false,"target_va":5380424288,"va":5379403954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37214653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379403888,"raw_hex":"e8bdd93702","size":5,"soft_fail":false,"target_va":5416619696,"va":5379405038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":40372,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":534,"record_offset":40488,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":534,"record_offset":42512,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":534,"record_offset":40800,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":42144,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":534,"record_offset":43316,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":534,"record_offset":44012,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":534,"record_offset":40660,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":11153,"source_line":2710,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":534,"record_offset":40688,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":534,"record_offset":43360,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":534,"record_offset":42856,"parent_record_offset":40168,"ancestor_record_offset":40168,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":40168,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41524,"parent_record_offset":41500,"ancestor_record_offset":40168,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":42392,"parent_record_offset":42368,"ancestor_record_offset":40168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":42416,"parent_record_offset":42392,"ancestor_record_offset":40168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":40168,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":42440,"parent_record_offset":42416,"ancestor_record_offset":40168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41476,"parent_record_offset":41312,"ancestor_record_offset":40168,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":42368,"parent_record_offset":42312,"ancestor_record_offset":40168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41220,"parent_record_offset":41176,"ancestor_record_offset":40168,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":42340,"parent_record_offset":42312,"ancestor_record_offset":40168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":42312,"parent_record_offset":42288,"ancestor_record_offset":40168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41312,"parent_record_offset":41264,"ancestor_record_offset":40168,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41264,"parent_record_offset":41220,"ancestor_record_offset":40168,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":42288,"parent_record_offset":42264,"ancestor_record_offset":40168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41364,"parent_record_offset":41312,"ancestor_record_offset":40168,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":43500,"parent_record_offset":43472,"ancestor_record_offset":40168,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":43528,"parent_record_offset":43500,"ancestor_record_offset":40168,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":40880,"parent_record_offset":40844,"ancestor_record_offset":40168,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":40920,"parent_record_offset":40880,"ancestor_record_offset":40168,"inlinee":32454,"source_line":708,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":41816,"parent_record_offset":41780,"ancestor_record_offset":40168,"inlinee":32456,"source_line":709,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41636,"parent_record_offset":41608,"ancestor_record_offset":40168,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41132,"parent_record_offset":41080,"ancestor_record_offset":40168,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41176,"parent_record_offset":41132,"ancestor_record_offset":40168,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":43472,"parent_record_offset":43444,"ancestor_record_offset":40168,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41000,"parent_record_offset":40960,"ancestor_record_offset":40168,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41916,"parent_record_offset":41868,"ancestor_record_offset":40168,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":534,"record_offset":44500,"parent_record_offset":44476,"ancestor_record_offset":40168,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":534,"record_offset":44988,"parent_record_offset":44964,"ancestor_record_offset":40168,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":534,"record_offset":42040,"parent_record_offset":41916,"ancestor_record_offset":40168,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":534,"record_offset":41608,"parent_record_offset":41080,"ancestor_record_offset":40168,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":534,"record_offset":41040,"parent_record_offset":41000,"ancestor_record_offset":40168,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":534,"record_offset":41080,"parent_record_offset":41040,"ancestor_record_offset":40168,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":534,"record_offset":43444,"parent_record_offset":43416,"ancestor_record_offset":40168,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":40168,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":41436,"parent_record_offset":41412,"ancestor_record_offset":40168,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":41692,"parent_record_offset":41664,"ancestor_record_offset":40168,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":41780,"parent_record_offset":41720,"ancestor_record_offset":40168,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":40960,"parent_record_offset":40920,"ancestor_record_offset":40168,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":41664,"parent_record_offset":41636,"ancestor_record_offset":40168,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":41868,"parent_record_offset":41780,"ancestor_record_offset":40168,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":534,"record_offset":40400,"parent_record_offset":40372,"ancestor_record_offset":40168,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":534,"record_offset":40424,"parent_record_offset":40400,"ancestor_record_offset":40168,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":41412,"parent_record_offset":41388,"ancestor_record_offset":40168,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":45132,"parent_record_offset":45100,"ancestor_record_offset":40168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":534,"record_offset":42536,"parent_record_offset":42512,"ancestor_record_offset":40168,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":43928,"parent_record_offset":43528,"ancestor_record_offset":40168,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":43952,"parent_record_offset":43928,"ancestor_record_offset":40168,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":42168,"parent_record_offset":42144,"ancestor_record_offset":40168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":40456,"parent_record_offset":40372,"ancestor_record_offset":40168,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":534,"record_offset":40564,"parent_record_offset":40488,"ancestor_record_offset":40168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":534,"record_offset":40592,"parent_record_offset":40488,"ancestor_record_offset":40168,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":534,"record_offset":40532,"parent_record_offset":40488,"ancestor_record_offset":40168,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":40620,"parent_record_offset":40592,"ancestor_record_offset":40168,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":41988,"parent_record_offset":41916,"ancestor_record_offset":40168,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":534,"record_offset":41720,"parent_record_offset":41692,"ancestor_record_offset":40168,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":43236,"parent_record_offset":43208,"ancestor_record_offset":40168,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":44320,"parent_record_offset":44132,"ancestor_record_offset":40168,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":42928,"parent_record_offset":42896,"ancestor_record_offset":40168,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":44408,"parent_record_offset":44380,"ancestor_record_offset":40168,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":43140,"parent_record_offset":43108,"ancestor_record_offset":40168,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":42952,"parent_record_offset":42928,"ancestor_record_offset":40168,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":43164,"parent_record_offset":43140,"ancestor_record_offset":40168,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":44344,"parent_record_offset":44320,"ancestor_record_offset":40168,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":534,"record_offset":44432,"parent_record_offset":44408,"ancestor_record_offset":40168,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":534,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":40168,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":534,"record_offset":40844,"parent_record_offset":40800,"ancestor_record_offset":40168,"inlinee":32453,"source_line":206,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":534,"record_offset":44036,"parent_record_offset":44012,"ancestor_record_offset":40168,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":40712,"parent_record_offset":40688,"ancestor_record_offset":40168,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":43388,"parent_record_offset":43360,"ancestor_record_offset":40168,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":534,"record_offset":43108,"parent_record_offset":43080,"ancestor_record_offset":40168,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":534,"record_offset":43080,"parent_record_offset":43052,"ancestor_record_offset":40168,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":534,"record_offset":44132,"parent_record_offset":44096,"ancestor_record_offset":40168,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":534,"record_offset":43208,"parent_record_offset":43016,"ancestor_record_offset":40168,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":534,"record_offset":43016,"parent_record_offset":42988,"ancestor_record_offset":40168,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":534,"record_offset":42988,"parent_record_offset":42856,"ancestor_record_offset":40168,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":534,"record_offset":44096,"parent_record_offset":44068,"ancestor_record_offset":40168,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":534,"record_offset":40736,"parent_record_offset":40712,"ancestor_record_offset":40168,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":534,"record_offset":43416,"parent_record_offset":43388,"ancestor_record_offset":40168,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":534,"record_offset":42896,"parent_record_offset":42856,"ancestor_record_offset":40168,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":534,"record_offset":44380,"parent_record_offset":44096,"ancestor_record_offset":40168,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":534,"record_offset":43052,"parent_record_offset":43016,"ancestor_record_offset":40168,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EC5
// 首条位置: {"module":534,"inlinee":32453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":40880,"parent_record_offset":40844,"ancestor_record_offset":40844,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":40844,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41524,"parent_record_offset":41500,"ancestor_record_offset":40844,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":40844,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":41476,"parent_record_offset":41312,"ancestor_record_offset":40844,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41220,"parent_record_offset":41176,"ancestor_record_offset":40844,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41312,"parent_record_offset":41264,"ancestor_record_offset":40844,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41264,"parent_record_offset":41220,"ancestor_record_offset":40844,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":534,"record_offset":41364,"parent_record_offset":41312,"ancestor_record_offset":40844,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":40920,"parent_record_offset":40880,"ancestor_record_offset":40844,"inlinee":32454,"source_line":708,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":41816,"parent_record_offset":41780,"ancestor_record_offset":40844,"inlinee":32456,"source_line":709,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41636,"parent_record_offset":41608,"ancestor_record_offset":40844,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41132,"parent_record_offset":41080,"ancestor_record_offset":40844,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41176,"parent_record_offset":41132,"ancestor_record_offset":40844,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41000,"parent_record_offset":40960,"ancestor_record_offset":40844,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":41916,"parent_record_offset":41868,"ancestor_record_offset":40844,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":534,"record_offset":42040,"parent_record_offset":41916,"ancestor_record_offset":40844,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":534,"record_offset":41608,"parent_record_offset":41080,"ancestor_record_offset":40844,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":534,"record_offset":41040,"parent_record_offset":41000,"ancestor_record_offset":40844,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":534,"record_offset":41080,"parent_record_offset":41040,"ancestor_record_offset":40844,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":40844,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":41436,"parent_record_offset":41412,"ancestor_record_offset":40844,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":41692,"parent_record_offset":41664,"ancestor_record_offset":40844,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":41780,"parent_record_offset":41720,"ancestor_record_offset":40844,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":40960,"parent_record_offset":40920,"ancestor_record_offset":40844,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":41664,"parent_record_offset":41636,"ancestor_record_offset":40844,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":41868,"parent_record_offset":41780,"ancestor_record_offset":40844,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":41412,"parent_record_offset":41388,"ancestor_record_offset":40844,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":41988,"parent_record_offset":41916,"ancestor_record_offset":40844,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":534,"record_offset":41720,"parent_record_offset":41692,"ancestor_record_offset":40844,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::event_paths
// 记录形式: compiled_procedure; instances=1
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":2536,"rva":11715168,"va":5380424288}
// 编译实例 RVAs: 0xb2c260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4745721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424288,"raw_hex":"e80796b7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380424404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4746097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424288,"raw_hex":"e88f94b7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380424796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11702202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424288,"raw_hex":"e846704dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380424389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098cf50
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1700924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424288,"raw_hex":"e8c40be6ff","size":5,"soft_fail":false,"target_va":5378723664,"va":5380424583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36196148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424288,"raw_hex":"e8344f2802","size":5,"soft_fail":false,"target_va":5416620769,"va":5380424616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":565,"record_offset":2860,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":565,"record_offset":2680,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3128,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":2536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":2536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":2536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":2740,"parent_record_offset":2708,"ancestor_record_offset":2536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":3352,"parent_record_offset":3296,"ancestor_record_offset":2536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":565,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":2536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":2536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":565,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2536,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":2536,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":2536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::classify
// 记录形式: compiled_procedure; instances=1
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":55936,"rva":11737712,"va":5380446832}
// 编译实例 RVAs: 0xb31a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4383606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380446832,"raw_hex":"e876e34200","size":5,"soft_fail":false,"target_va":5384830608,"va":5380446997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_texture_source_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2471550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380446832,"raw_hex":"e87eb62500","size":5,"soft_fail":false,"target_va":5382918832,"va":5380447277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":56972,"parent_record_offset":55936,"ancestor_record_offset":55936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":56080,"parent_record_offset":55936,"ancestor_record_offset":55936,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":57284,"parent_record_offset":55936,"ancestor_record_offset":55936,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":56804,"parent_record_offset":55936,"ancestor_record_offset":55936,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":56312,"parent_record_offset":56284,"ancestor_record_offset":55936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":56144,"parent_record_offset":56108,"ancestor_record_offset":55936,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":56108,"parent_record_offset":56080,"ancestor_record_offset":55936,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":56560,"parent_record_offset":56532,"ancestor_record_offset":55936,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":56336,"parent_record_offset":56312,"ancestor_record_offset":55936,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":56360,"parent_record_offset":56336,"ancestor_record_offset":55936,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":57332,"parent_record_offset":57308,"ancestor_record_offset":55936,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":56508,"parent_record_offset":56472,"ancestor_record_offset":55936,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":56284,"parent_record_offset":56256,"ancestor_record_offset":55936,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":565,"record_offset":56592,"parent_record_offset":56532,"ancestor_record_offset":55936,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":565,"record_offset":56532,"parent_record_offset":56508,"ancestor_record_offset":55936,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":565,"record_offset":56628,"parent_record_offset":56472,"ancestor_record_offset":55936,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":565,"record_offset":56656,"parent_record_offset":56628,"ancestor_record_offset":55936,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":56200,"parent_record_offset":56172,"ancestor_record_offset":55936,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":565,"record_offset":56444,"parent_record_offset":56420,"ancestor_record_offset":55936,"inlinee":22833,"source_line":657,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":565,"record_offset":56420,"parent_record_offset":56108,"ancestor_record_offset":55936,"inlinee":22809,"source_line":511,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":56472,"parent_record_offset":56444,"ancestor_record_offset":55936,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":56256,"parent_record_offset":56228,"ancestor_record_offset":55936,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":57308,"parent_record_offset":57284,"ancestor_record_offset":55936,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":56228,"parent_record_offset":56200,"ancestor_record_offset":55936,"inlinee":15546,"source_line":593,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":56996,"parent_record_offset":56972,"ancestor_record_offset":55936,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::classify::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":56172,"parent_record_offset":56144,"ancestor_record_offset":55936,"inlinee":35546,"source_line":248,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::classify::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 248
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8ADA
// 首条位置: {"module":565,"inlinee":35546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":56200,"parent_record_offset":56172,"ancestor_record_offset":56172,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":56312,"parent_record_offset":56284,"ancestor_record_offset":56172,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":56336,"parent_record_offset":56312,"ancestor_record_offset":56172,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":56360,"parent_record_offset":56336,"ancestor_record_offset":56172,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":56284,"parent_record_offset":56256,"ancestor_record_offset":56172,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":56256,"parent_record_offset":56228,"ancestor_record_offset":56172,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":56228,"parent_record_offset":56200,"ancestor_record_offset":56172,"inlinee":15546,"source_line":593,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::mod_hot_reload_apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":565,"record_offset":4236,"rva":11715808,"va":5380424928}
// 编译实例 RVAs: 0xb2c4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4219806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e89e634000","size":5,"soft_fail":false,"target_va":5384644912,"va":5380425101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4227257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8b9804000","size":5,"soft_fail":false,"target_va":5384653152,"va":5380425890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36181204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8d4142802","size":5,"soft_fail":false,"target_va":5416616112,"va":5380434903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36187425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8212d2802","size":5,"soft_fail":false,"target_va":5416616304,"va":5380428874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4214273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8014e4000","size":5,"soft_fail":false,"target_va":5384648448,"va":5380434170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4220496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e850664000","size":5,"soft_fail":false,"target_va":5384648032,"va":5380427531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t4182939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e89bd33f00","size":5,"soft_fail":false,"target_va":5384609408,"va":5380426464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4753277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e88378b7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380431960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=42
//     evidence={"assembly":"\tcallq\t-4747553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8df8eb7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380426252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11709760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8c0524dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380431947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=5
//     target_aliases=27 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11213847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8e9e354ff","size":5,"soft_fail":false,"target_va":5369224048,"va":5380437890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11195510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e88a2b55ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5380435953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11176109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8537755ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5380441224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008e5a0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11148582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8dae255ff","size":5,"soft_fail":false,"target_va":5369292192,"va":5380440769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=7
//     target_aliases=4 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11090540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e894c556ff","size":5,"soft_fail":false,"target_va":5369339792,"va":5380430327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10924734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8424d59ff","size":5,"soft_fail":false,"target_va":5369512480,"va":5380437209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401550d0
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10334276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8bc4f62ff","size":5,"soft_fail":false,"target_va":5370106064,"va":5380440335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7460450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e89e298eff","size":5,"soft_fail":false,"target_va":5372967648,"va":5380428093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6333647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8315b9fff","size":5,"soft_fail":false,"target_va":5374098704,"va":5380432346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140927290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2124421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e87b95dfff","size":5,"soft_fail":false,"target_va":5378306704,"va":5380431120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096cc40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1842259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8ade3e3ff","size":5,"soft_fail":false,"target_va":5378591808,"va":5380434062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140976630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1810126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e83261e4ff","size":5,"soft_fail":false,"target_va":5378631216,"va":5380441337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1703399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e81902e6ff","size":5,"soft_fail":false,"target_va":5378733248,"va":5380436642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409911c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1691864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8282fe6ff","size":5,"soft_fail":false,"target_va":5378740672,"va":5380432531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1637136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f004e7ff","size":5,"soft_fail":false,"target_va":5378801568,"va":5380438699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1d970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8c10fffff","size":5,"soft_fail":false,"target_va":5380364656,"va":5380426154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1529417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e849561700","size":5,"soft_fail":false,"target_va":5381954800,"va":5380425378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca5960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1536837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e845731700","size":5,"soft_fail":false,"target_va":5381970272,"va":5380433430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fa610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11323212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e84cc7ac00","size":5,"soft_fail":false,"target_va":5391754768,"va":5380431551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fa9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11324125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8ddcaac00","size":5,"soft_fail":false,"target_va":5391755760,"va":5380431630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141610e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11418850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8e23cae00","size":5,"soft_fail":false,"target_va":5391847024,"va":5380428169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12344529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8d15cbc00","size":5,"soft_fail":false,"target_va":5392780768,"va":5380436234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ffd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12393433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8d91bbd00","size":5,"soft_fail":false,"target_va":5392825616,"va":5380432178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14179c8f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13038879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e81ff5c600","size":5,"soft_fail":false,"target_va":5393467632,"va":5380428748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417aac00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13090145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e861bdc700","size":5,"soft_fail":false,"target_va":5393525760,"va":5380435610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ab2c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13095654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8e6d2c700","size":5,"soft_fail":false,"target_va":5393527488,"va":5380431829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b4650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13131854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e84e60c800","size":5,"soft_fail":false,"target_va":5393565264,"va":5380433405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180dca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13499376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f0fbcd00","size":5,"soft_fail":false,"target_va":5393931424,"va":5380432043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a86010
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16088769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8c17ef500","size":5,"soft_fail":false,"target_va":5396520976,"va":5380432202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c36a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17862645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f58f1001","size":5,"soft_fail":false,"target_va":5398293120,"va":5380430470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35916603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e83b0b2402","size":5,"soft_fail":false,"target_va":5416350720,"va":5380434112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35924736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8002b2402","size":5,"soft_fail":false,"target_va":5416351392,"va":5380426651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35943072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8a0722402","size":5,"soft_fail":false,"target_va":5416382848,"va":5380439771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35949942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8768d2402","size":5,"soft_fail":false,"target_va":5416385024,"va":5380435077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35968684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8acd62402","size":5,"soft_fail":false,"target_va":5416398672,"va":5380429983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7f170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35983806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8be112502","size":5,"soft_fail":false,"target_va":5416415600,"va":5380431789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d915b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36054827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e82b272602","size":5,"soft_fail":false,"target_va":5416490416,"va":5380435584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9fd20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36114236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e83c0f2702","size":5,"soft_fail":false,"target_va":5416549664,"va":5380435423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36631026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f2f12e02","size":5,"soft_fail":false,"target_va":5417057040,"va":5380426009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36626426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8fadf2e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5380430625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35939592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e808652402","size":5,"soft_fail":false,"target_va":5416370864,"va":5380431267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36184944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e870232802","size":5,"soft_fail":false,"target_va":5416620794,"va":5380435845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2776104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8285c2a00","size":5,"soft_fail":false,"target_va":5383206064,"va":5380429955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4194979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8a3024000","size":5,"soft_fail":false,"target_va":5384624080,"va":5380429096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::store::try_locked_store
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13474908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e85c9ccd00","size":5,"soft_fail":false,"target_va":5393902688,"va":5380427775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36186258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e892282802","size":5,"soft_fail":false,"target_va":5416621408,"va":5380435145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36186653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e81d2a2802","size":5,"soft_fail":false,"target_va":5416621632,"va":5380434974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8a16e0000","size":5,"soft_fail":false,"target_va":5380455264,"va":5380426938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36185660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e83c262802","size":5,"soft_fail":false,"target_va":5416621376,"va":5380435711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::ClutterPackObject::id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1470419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8d36f1600","size":5,"soft_fail":false,"target_va":5381902240,"va":5380431816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::external_ids_for_mod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1018130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8ee76f0ff","size":5,"soft_fail":false,"target_va":5379413360,"va":5380431485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1018917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8db73f0ff","size":5,"soft_fail":false,"target_va":5379414368,"va":5380433280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::discover_clutter_part
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16447087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e86ff6fa00","size":5,"soft_fail":false,"target_va":5396878032,"va":5380430940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::install_built_clutter_packs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16465867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8cb3ffb00","size":5,"soft_fail":false,"target_va":5396899440,"va":5380433568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::resync_clutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16439747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8c3d9fa00","size":5,"soft_fail":false,"target_va":5396873760,"va":5380434008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::unload_clutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16442196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e854e3fa00","size":5,"soft_fail":false,"target_va":5396875136,"va":5380432935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::reload_mod_textures
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1350564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e85c64ebff","size":5,"soft_fail":false,"target_va":5379077584,"va":5380428143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::mod_hot_reload::classify
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t21048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e838520000","size":5,"soft_fail":false,"target_va":5380446832,"va":5380425779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::swap_replacement_mods::reload_assets_for_changed_paths
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2246842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e846b7ddff","size":5,"soft_fail":false,"target_va":5378181568,"va":5380428405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3536312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8b8f53500","size":5,"soft_fail":false,"target_va":5383963792,"va":5380427475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3544820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f4163600","size":5,"soft_fail":false,"target_va":5383972224,"va":5380427399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3535721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e869f33500","size":5,"soft_fail":false,"target_va":5383964912,"va":5380429186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3535912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e828f43500","size":5,"soft_fail":false,"target_va":5383962816,"va":5380426899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3302646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8f6643200","size":5,"soft_fail":false,"target_va":5383730768,"va":5380428117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36184778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8ca222802","size":5,"soft_fail":false,"target_va":5416619696,"va":5380434913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4294932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e814894100","size":5,"soft_fail":false,"target_va":5384726688,"va":5380431751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36184242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8b2202802","size":5,"soft_fail":false,"target_va":5416619360,"va":5380435113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::mod_parts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2518597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e8456e2600","size":5,"soft_fail":false,"target_va":5382949456,"va":5380430854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2473479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e807be2500","size":5,"soft_fail":false,"target_va":5382903888,"va":5380430404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::overlay_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2456682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e86a7c2500","size":5,"soft_fail":false,"target_va":5382884768,"va":5380428081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::refresh_active_overlay
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2490773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380424928,"raw_hex":"e895012600","size":5,"soft_fail":false,"target_va":5382918784,"va":5380428006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":4472,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":12020,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":6244,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":7032,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":565,"record_offset":14596,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":565,"record_offset":41904,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":9128,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":5000,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":5748,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":565,"record_offset":4956,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::mod_hot_reload::is_replacement_watched
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":7864,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":35525,"source_line":384,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::mod_hot_reload::reload_changed_clutter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":23180,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":35533,"source_line":459,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::mod_hot_reload::reload_changed_replacements
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":14896,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":35530,"source_line":398,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":9184,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":9044,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":13480,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":12076,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":12464,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":6160,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":18704,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":6300,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":7276,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":4500,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":18648,"parent_record_offset":4236,"ancestor_record_offset":4236,"inlinee":12499,"source_line":735,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":4236,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33152,"parent_record_offset":33128,"ancestor_record_offset":4236,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":4236,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":4236,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33176,"parent_record_offset":33152,"ancestor_record_offset":4236,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":8800,"parent_record_offset":8776,"ancestor_record_offset":4236,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33104,"parent_record_offset":32960,"ancestor_record_offset":4236,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":8728,"parent_record_offset":8672,"ancestor_record_offset":4236,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":7152,"parent_record_offset":7120,"ancestor_record_offset":4236,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32900,"parent_record_offset":32868,"ancestor_record_offset":4236,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":8700,"parent_record_offset":8672,"ancestor_record_offset":4236,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":4236,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":4236,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":4236,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32960,"parent_record_offset":32928,"ancestor_record_offset":4236,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32928,"parent_record_offset":32900,"ancestor_record_offset":4236,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":4236,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32992,"parent_record_offset":32960,"ancestor_record_offset":4236,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":42456,"parent_record_offset":42420,"ancestor_record_offset":4236,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":33452,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":15612,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":23696,"parent_record_offset":23672,"ancestor_record_offset":4236,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":7120,"parent_record_offset":7068,"ancestor_record_offset":4236,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33820,"parent_record_offset":33788,"ancestor_record_offset":4236,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33788,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33260,"parent_record_offset":33236,"ancestor_record_offset":4236,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":15696,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":24128,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":29824,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":7068,"parent_record_offset":7032,"ancestor_record_offset":4236,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32836,"parent_record_offset":32784,"ancestor_record_offset":4236,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32868,"parent_record_offset":32836,"ancestor_record_offset":4236,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":4236,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":18752,"parent_record_offset":18728,"ancestor_record_offset":4236,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":31444,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":36896,"parent_record_offset":36872,"ancestor_record_offset":4236,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":4236,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26836,"parent_record_offset":26792,"ancestor_record_offset":4236,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26792,"parent_record_offset":26760,"ancestor_record_offset":4236,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":4236,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26908,"parent_record_offset":26884,"ancestor_record_offset":4236,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":565,"record_offset":36952,"parent_record_offset":36896,"ancestor_record_offset":4236,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":565,"record_offset":33236,"parent_record_offset":32784,"ancestor_record_offset":4236,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":565,"record_offset":32744,"parent_record_offset":32704,"ancestor_record_offset":4236,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":565,"record_offset":32784,"parent_record_offset":32744,"ancestor_record_offset":4236,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":565,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":4236,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":565,"record_offset":23848,"parent_record_offset":23824,"ancestor_record_offset":4236,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":565,"record_offset":14624,"parent_record_offset":14596,"ancestor_record_offset":4236,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":565,"record_offset":41928,"parent_record_offset":41904,"ancestor_record_offset":4236,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":565,"record_offset":41992,"parent_record_offset":41928,"ancestor_record_offset":4236,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::build_clutter_packs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":565,"record_offset":33540,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":35538,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":565,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":4236,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":13836,"parent_record_offset":13808,"ancestor_record_offset":4236,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":5568,"parent_record_offset":5540,"ancestor_record_offset":4236,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":5468,"parent_record_offset":5264,"ancestor_record_offset":4236,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":13776,"parent_record_offset":13744,"ancestor_record_offset":4236,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":13808,"parent_record_offset":13776,"ancestor_record_offset":4236,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":13924,"parent_record_offset":13808,"ancestor_record_offset":4236,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":4236,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":15808,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":8284,"parent_record_offset":8132,"ancestor_record_offset":4236,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":565,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":4236,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":15152,"parent_record_offset":15120,"ancestor_record_offset":4236,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":565,"record_offset":38448,"parent_record_offset":38340,"ancestor_record_offset":4236,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":565,"record_offset":38176,"parent_record_offset":38148,"ancestor_record_offset":4236,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":38340,"parent_record_offset":38232,"ancestor_record_offset":4236,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":38204,"parent_record_offset":38176,"ancestor_record_offset":4236,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":29272,"parent_record_offset":29248,"ancestor_record_offset":4236,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":18832,"parent_record_offset":18808,"ancestor_record_offset":4236,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":4236,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":29248,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":15380,"parent_record_offset":15284,"ancestor_record_offset":4236,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":565,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":4236,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":565,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":4236,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":565,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":4236,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":565,"record_offset":26860,"parent_record_offset":26836,"ancestor_record_offset":4236,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15640,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":26692,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15188,"parent_record_offset":15056,"ancestor_record_offset":4236,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":33308,"parent_record_offset":33284,"ancestor_record_offset":4236,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":4236,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":18808,"parent_record_offset":18728,"ancestor_record_offset":4236,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":8368,"parent_record_offset":8132,"ancestor_record_offset":4236,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":10696,"parent_record_offset":10660,"ancestor_record_offset":4236,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":35352,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":565,"record_offset":26604,"parent_record_offset":26572,"ancestor_record_offset":4236,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":565,"record_offset":26628,"parent_record_offset":26604,"ancestor_record_offset":4236,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":4236,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":6808,"parent_record_offset":6720,"ancestor_record_offset":4236,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4236,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":6776,"parent_record_offset":6748,"ancestor_record_offset":4236,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":15548,"parent_record_offset":15284,"ancestor_record_offset":4236,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":16108,"parent_record_offset":16076,"ancestor_record_offset":4236,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":5596,"parent_record_offset":5540,"ancestor_record_offset":4236,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":4236,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":4236,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":565,"record_offset":26720,"parent_record_offset":26692,"ancestor_record_offset":4236,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":15428,"parent_record_offset":15404,"ancestor_record_offset":4236,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":5176,"parent_record_offset":5140,"ancestor_record_offset":4236,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":29448,"parent_record_offset":29420,"ancestor_record_offset":4236,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":4236,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":28712,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":28736,"parent_record_offset":28712,"ancestor_record_offset":4236,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":4236,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":29420,"parent_record_offset":29248,"ancestor_record_offset":4236,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":5296,"parent_record_offset":5264,"ancestor_record_offset":4236,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":565,"record_offset":33616,"parent_record_offset":33568,"ancestor_record_offset":4236,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":6584,"parent_record_offset":6492,"ancestor_record_offset":4236,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":14272,"parent_record_offset":13596,"ancestor_record_offset":4236,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":26980,"parent_record_offset":26956,"ancestor_record_offset":4236,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=203
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":4236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":4236,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":4236,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":26956,"parent_record_offset":26932,"ancestor_record_offset":4236,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":26496,"parent_record_offset":26100,"ancestor_record_offset":4236,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":5384,"parent_record_offset":5264,"ancestor_record_offset":4236,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":6840,"parent_record_offset":6680,"ancestor_record_offset":4236,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":15352,"parent_record_offset":15284,"ancestor_record_offset":4236,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":42488,"parent_record_offset":42456,"ancestor_record_offset":4236,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":4236,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":26932,"parent_record_offset":26908,"ancestor_record_offset":4236,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":12864,"parent_record_offset":12840,"ancestor_record_offset":4236,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":4236,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":4236,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":4236,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":7240,"parent_record_offset":7068,"ancestor_record_offset":4236,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":4236,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":4236,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":4236,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":4236,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":4236,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":4236,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":4236,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":4236,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":4236,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":7976,"parent_record_offset":7864,"ancestor_record_offset":4236,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":29296,"parent_record_offset":29272,"ancestor_record_offset":4236,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":4236,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":8004,"parent_record_offset":7976,"ancestor_record_offset":4236,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":12792,"parent_record_offset":12676,"ancestor_record_offset":4236,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":38636,"parent_record_offset":38604,"ancestor_record_offset":4236,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":4236,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":12500,"parent_record_offset":12464,"ancestor_record_offset":4236,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":29628,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":15542,"source_line":2443,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":565,"record_offset":12644,"parent_record_offset":12612,"ancestor_record_offset":4236,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":565,"record_offset":12612,"parent_record_offset":12572,"ancestor_record_offset":4236,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":4236,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":4236,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":4236,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":4236,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":4236,"inlinee":15545,"source_line":1017,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":13116,"parent_record_offset":12572,"ancestor_record_offset":4236,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12180,"parent_record_offset":12148,"ancestor_record_offset":4236,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":4236,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12304,"parent_record_offset":12272,"ancestor_record_offset":4236,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12360,"parent_record_offset":12304,"ancestor_record_offset":4236,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":4236,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":29508,"parent_record_offset":29484,"ancestor_record_offset":4236,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12120,"parent_record_offset":12076,"ancestor_record_offset":4236,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":4236,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":565,"record_offset":12676,"parent_record_offset":12644,"ancestor_record_offset":4236,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":565,"record_offset":12960,"parent_record_offset":12676,"ancestor_record_offset":4236,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":565,"record_offset":12932,"parent_record_offset":12676,"ancestor_record_offset":4236,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":565,"record_offset":12736,"parent_record_offset":12676,"ancestor_record_offset":4236,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":4236,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":6000,"parent_record_offset":5812,"ancestor_record_offset":4236,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4568,"parent_record_offset":4536,"ancestor_record_offset":4236,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":6092,"parent_record_offset":6060,"ancestor_record_offset":4236,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33480,"parent_record_offset":33452,"ancestor_record_offset":4236,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23732,"parent_record_offset":23696,"ancestor_record_offset":4236,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4780,"parent_record_offset":4748,"ancestor_record_offset":4236,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33504,"parent_record_offset":33480,"ancestor_record_offset":4236,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":4236,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4236,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":4236,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":4236,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":4236,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23788,"parent_record_offset":23696,"ancestor_record_offset":4236,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::ClutterRegistry::forget_external
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":565,"record_offset":38604,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":32521,"source_line":362,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_clutter_packs::build_requests
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":565,"record_offset":32620,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":35536,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::is_replacement_watched::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":8060,"parent_record_offset":7976,"ancestor_record_offset":4236,"inlinee":35527,"source_line":387,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":4236,"inlinee":35534,"source_line":511,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":36844,"parent_record_offset":36820,"ancestor_record_offset":4236,"inlinee":35539,"source_line":585,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":4236,"inlinee":35540,"source_line":590,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":565,"record_offset":33568,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":7492,"parent_record_offset":7456,"ancestor_record_offset":4236,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":7524,"parent_record_offset":7492,"ancestor_record_offset":4236,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":7612,"parent_record_offset":7456,"ancestor_record_offset":4236,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":7456,"parent_record_offset":7408,"ancestor_record_offset":4236,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":565,"record_offset":6528,"parent_record_offset":6492,"ancestor_record_offset":4236,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":4236,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38772,"parent_record_offset":38748,"ancestor_record_offset":4236,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38796,"parent_record_offset":38772,"ancestor_record_offset":4236,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15092,"parent_record_offset":15056,"ancestor_record_offset":4236,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":6364,"parent_record_offset":6332,"ancestor_record_offset":4236,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":4236,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15056,"parent_record_offset":15028,"ancestor_record_offset":4236,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":4236,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38824,"parent_record_offset":38796,"ancestor_record_offset":4236,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":11120,"parent_record_offset":10660,"ancestor_record_offset":4236,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":7700,"parent_record_offset":7408,"ancestor_record_offset":4236,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIter::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":10660,"parent_record_offset":10620,"ancestor_record_offset":4236,"inlinee":4991,"source_line":3818,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":4236,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":7408,"parent_record_offset":7376,"ancestor_record_offset":4236,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":4236,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6652,"parent_record_offset":6396,"ancestor_record_offset":4236,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6436,"parent_record_offset":6396,"ancestor_record_offset":4236,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":15120,"parent_record_offset":15092,"ancestor_record_offset":4236,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17512,"parent_record_offset":17480,"ancestor_record_offset":4236,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16564,"parent_record_offset":16536,"ancestor_record_offset":4236,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16504,"parent_record_offset":16440,"ancestor_record_offset":4236,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16440,"parent_record_offset":16408,"ancestor_record_offset":4236,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17480,"parent_record_offset":16440,"ancestor_record_offset":4236,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16476,"parent_record_offset":16440,"ancestor_record_offset":4236,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":4236,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6720,"parent_record_offset":6680,"ancestor_record_offset":4236,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":4236,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":6396,"parent_record_offset":6364,"ancestor_record_offset":4236,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":4236,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":10620,"parent_record_offset":10588,"ancestor_record_offset":4236,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":4236,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":4236,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":15028,"parent_record_offset":15000,"ancestor_record_offset":4236,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":6332,"parent_record_offset":6300,"ancestor_record_offset":4236,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":4236,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":4236,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":15864,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":4236,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":4236,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":15724,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":13516,"parent_record_offset":13480,"ancestor_record_offset":4236,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":19792,"parent_record_offset":19760,"ancestor_record_offset":4236,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":20272,"parent_record_offset":20056,"ancestor_record_offset":4236,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":4236,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":13744,"parent_record_offset":13596,"ancestor_record_offset":4236,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":565,"record_offset":13960,"parent_record_offset":13744,"ancestor_record_offset":4236,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":565,"record_offset":38748,"parent_record_offset":38604,"ancestor_record_offset":4236,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":24928,"parent_record_offset":24224,"ancestor_record_offset":4236,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":15000,"parent_record_offset":14972,"ancestor_record_offset":4236,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":4236,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":25448,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":26004,"parent_record_offset":23180,"ancestor_record_offset":4236,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":24224,"parent_record_offset":24184,"ancestor_record_offset":4236,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":4236,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":4236,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":4236,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":5540,"parent_record_offset":5140,"ancestor_record_offset":4236,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":4236,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":565,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":4236,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":565,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":4236,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8200,"parent_record_offset":8132,"ancestor_record_offset":4236,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8172,"parent_record_offset":8132,"ancestor_record_offset":4236,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$148::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":4236,"inlinee":35529,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":4236,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":4236,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":5812,"parent_record_offset":5776,"ancestor_record_offset":4236,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":4848,"parent_record_offset":4656,"ancestor_record_offset":4236,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":565,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":4236,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":4628,"parent_record_offset":4500,"ancestor_record_offset":4236,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":565,"record_offset":5776,"parent_record_offset":5748,"ancestor_record_offset":4236,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":565,"record_offset":4536,"parent_record_offset":4500,"ancestor_record_offset":4236,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":565,"record_offset":6060,"parent_record_offset":5776,"ancestor_record_offset":4236,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":4236,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":4236,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":565,"record_offset":4692,"parent_record_offset":4656,"ancestor_record_offset":4236,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":5140,"parent_record_offset":5112,"ancestor_record_offset":4236,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":4236,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::overlay::ChangedPaths::extend_paths
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":565,"record_offset":14972,"parent_record_offset":14896,"ancestor_record_offset":4236,"inlinee":35532,"source_line":405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::is_replacement_watched
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AC5
// 首条位置: {"module":565,"inlinee":35525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":7976,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":8284,"parent_record_offset":8132,"ancestor_record_offset":7864,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":565,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":7864,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":8368,"parent_record_offset":8132,"ancestor_record_offset":7864,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":7864,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":7864,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":7864,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7864,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7864,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":7864,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":8004,"parent_record_offset":7976,"ancestor_record_offset":7864,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::is_replacement_watched::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":8060,"parent_record_offset":7976,"ancestor_record_offset":7864,"inlinee":35527,"source_line":387,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8200,"parent_record_offset":8132,"ancestor_record_offset":7864,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8172,"parent_record_offset":8132,"ancestor_record_offset":7864,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$148::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":7864,"inlinee":35529,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":7864,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":7864,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::is_replacement_watched::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AC7
// 首条位置: {"module":565,"inlinee":35527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$148::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":8060,"inlinee":35529,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":8284,"parent_record_offset":8132,"ancestor_record_offset":8060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":565,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":8060,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":8368,"parent_record_offset":8132,"ancestor_record_offset":8060,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":8060,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":8060,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":8060,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8200,"parent_record_offset":8132,"ancestor_record_offset":8060,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8172,"parent_record_offset":8132,"ancestor_record_offset":8060,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":8060,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":565,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":8060,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_replacements
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8ACA
// 首条位置: {"module":565,"inlinee":35530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":15612,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":15696,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":15808,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15640,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":15976,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":15864,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":15724,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::ChangedPaths::extend_paths
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":565,"record_offset":14972,"parent_record_offset":14896,"ancestor_record_offset":14896,"inlinee":35532,"source_line":405,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":16164,"parent_record_offset":16140,"ancestor_record_offset":14896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":14896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":16212,"parent_record_offset":16188,"ancestor_record_offset":14896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":16140,"parent_record_offset":16048,"ancestor_record_offset":14896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":16076,"parent_record_offset":16048,"ancestor_record_offset":14896,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":14896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":14896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":14896,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":565,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":14896,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":15152,"parent_record_offset":15120,"ancestor_record_offset":14896,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":14896,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":15380,"parent_record_offset":15284,"ancestor_record_offset":14896,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15188,"parent_record_offset":15056,"ancestor_record_offset":14896,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":14896,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":17572,"parent_record_offset":17544,"ancestor_record_offset":14896,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":17600,"parent_record_offset":17572,"ancestor_record_offset":14896,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":15548,"parent_record_offset":15284,"ancestor_record_offset":14896,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":16108,"parent_record_offset":16076,"ancestor_record_offset":14896,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":16864,"parent_record_offset":16840,"ancestor_record_offset":14896,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":15428,"parent_record_offset":15404,"ancestor_record_offset":14896,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":15832,"parent_record_offset":15808,"ancestor_record_offset":14896,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":16676,"parent_record_offset":16592,"ancestor_record_offset":14896,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":14896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":14896,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":14896,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":15352,"parent_record_offset":15284,"ancestor_record_offset":14896,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":15324,"parent_record_offset":15284,"ancestor_record_offset":14896,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":14896,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":14896,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":15772,"parent_record_offset":15748,"ancestor_record_offset":14896,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":16808,"parent_record_offset":16780,"ancestor_record_offset":14896,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":16840,"parent_record_offset":16808,"ancestor_record_offset":14896,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":16780,"parent_record_offset":16740,"ancestor_record_offset":14896,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":565,"record_offset":16620,"parent_record_offset":16592,"ancestor_record_offset":14896,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15092,"parent_record_offset":15056,"ancestor_record_offset":14896,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15056,"parent_record_offset":15028,"ancestor_record_offset":14896,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":14896,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17108,"parent_record_offset":16504,"ancestor_record_offset":14896,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16960,"parent_record_offset":16740,"ancestor_record_offset":14896,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16592,"parent_record_offset":16536,"ancestor_record_offset":14896,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":14896,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":15120,"parent_record_offset":15092,"ancestor_record_offset":14896,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17512,"parent_record_offset":17480,"ancestor_record_offset":14896,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16564,"parent_record_offset":16536,"ancestor_record_offset":14896,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16504,"parent_record_offset":16440,"ancestor_record_offset":14896,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16440,"parent_record_offset":16408,"ancestor_record_offset":14896,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17480,"parent_record_offset":16440,"ancestor_record_offset":14896,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16476,"parent_record_offset":16440,"ancestor_record_offset":14896,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16536,"parent_record_offset":16504,"ancestor_record_offset":14896,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":17544,"parent_record_offset":17512,"ancestor_record_offset":14896,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16408,"parent_record_offset":16380,"ancestor_record_offset":14896,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":16712,"parent_record_offset":16504,"ancestor_record_offset":14896,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":15028,"parent_record_offset":15000,"ancestor_record_offset":14896,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":14896,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":565,"record_offset":15888,"parent_record_offset":15864,"ancestor_record_offset":14896,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":14896,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":15000,"parent_record_offset":14972,"ancestor_record_offset":14896,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_replacements::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 426
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DDC
// 首条位置: {"module":1643,"inlinee":81372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1643,"record_offset":669008,"parent_record_offset":668984,"ancestor_record_offset":668984,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":669104,"parent_record_offset":669072,"ancestor_record_offset":668984,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":669128,"parent_record_offset":669104,"ancestor_record_offset":668984,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":669152,"parent_record_offset":669128,"ancestor_record_offset":668984,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":669072,"parent_record_offset":669040,"ancestor_record_offset":668984,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1643,"record_offset":669040,"parent_record_offset":669008,"ancestor_record_offset":668984,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_replacements::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71A6
// 首条位置: {"module":508,"inlinee":29094}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$93::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1643,"record_offset":669444,"parent_record_offset":669252,"ancestor_record_offset":669252,"inlinee":18157,"source_line":2670,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1643,"record_offset":669276,"parent_record_offset":669252,"ancestor_record_offset":669252,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":669468,"parent_record_offset":669444,"ancestor_record_offset":669252,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":669348,"parent_record_offset":669324,"ancestor_record_offset":669252,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":669372,"parent_record_offset":669348,"ancestor_record_offset":669252,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":669396,"parent_record_offset":669372,"ancestor_record_offset":669252,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":669324,"parent_record_offset":669300,"ancestor_record_offset":669252,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1643,"record_offset":669300,"parent_record_offset":669276,"ancestor_record_offset":669252,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1643,"record_offset":669492,"parent_record_offset":669468,"ancestor_record_offset":669252,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 459
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8ACD
// 首条位置: {"module":565,"inlinee":35533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":33452,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":35916,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33788,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32592,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":24128,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":29824,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":26572,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":31444,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] clutter_pack_pipe::build_clutter_packs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":565,"record_offset":33540,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":35538,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":33732,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":29484,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":29248,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":24184,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":26692,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":35352,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":28712,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":23644,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":565,"record_offset":29628,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":15542,"source_line":2443,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterRegistry::forget_external
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":565,"record_offset":38604,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":32521,"source_line":362,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::build_requests
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":565,"record_offset":32620,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":35536,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":565,"record_offset":33568,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":565,"record_offset":33648,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":25448,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":26004,"parent_record_offset":23180,"ancestor_record_offset":23180,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":23180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33152,"parent_record_offset":33128,"ancestor_record_offset":23180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":23180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":27312,"parent_record_offset":27288,"ancestor_record_offset":23180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33176,"parent_record_offset":33152,"ancestor_record_offset":23180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":27340,"parent_record_offset":27312,"ancestor_record_offset":23180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33104,"parent_record_offset":32960,"ancestor_record_offset":23180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":27264,"parent_record_offset":27168,"ancestor_record_offset":23180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":29920,"parent_record_offset":29888,"ancestor_record_offset":23180,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32900,"parent_record_offset":32868,"ancestor_record_offset":23180,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":27200,"parent_record_offset":27168,"ancestor_record_offset":23180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":27168,"parent_record_offset":27140,"ancestor_record_offset":23180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":29968,"parent_record_offset":29944,"ancestor_record_offset":23180,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":29944,"parent_record_offset":29920,"ancestor_record_offset":23180,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32960,"parent_record_offset":32928,"ancestor_record_offset":23180,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32928,"parent_record_offset":32900,"ancestor_record_offset":23180,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":27140,"parent_record_offset":27112,"ancestor_record_offset":23180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32992,"parent_record_offset":32960,"ancestor_record_offset":23180,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":565,"record_offset":23696,"parent_record_offset":23672,"ancestor_record_offset":23180,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":29888,"parent_record_offset":29848,"ancestor_record_offset":23180,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33820,"parent_record_offset":33788,"ancestor_record_offset":23180,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33260,"parent_record_offset":33236,"ancestor_record_offset":23180,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":29848,"parent_record_offset":29824,"ancestor_record_offset":23180,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32836,"parent_record_offset":32784,"ancestor_record_offset":23180,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32868,"parent_record_offset":32836,"ancestor_record_offset":23180,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32704,"parent_record_offset":32664,"ancestor_record_offset":23180,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":36896,"parent_record_offset":36872,"ancestor_record_offset":23180,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26836,"parent_record_offset":26792,"ancestor_record_offset":23180,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26792,"parent_record_offset":26760,"ancestor_record_offset":23180,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":23180,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":565,"record_offset":26908,"parent_record_offset":26884,"ancestor_record_offset":23180,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":565,"record_offset":36952,"parent_record_offset":36896,"ancestor_record_offset":23180,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":565,"record_offset":33236,"parent_record_offset":32784,"ancestor_record_offset":23180,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":565,"record_offset":32744,"parent_record_offset":32704,"ancestor_record_offset":23180,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":565,"record_offset":32784,"parent_record_offset":32744,"ancestor_record_offset":23180,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":565,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":23180,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":565,"record_offset":23848,"parent_record_offset":23824,"ancestor_record_offset":23180,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":565,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":23180,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":24788,"parent_record_offset":24760,"ancestor_record_offset":23180,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":565,"record_offset":24688,"parent_record_offset":24484,"ancestor_record_offset":23180,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":565,"record_offset":29532,"parent_record_offset":29508,"ancestor_record_offset":23180,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":565,"record_offset":25712,"parent_record_offset":25688,"ancestor_record_offset":23180,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":565,"record_offset":25104,"parent_record_offset":25072,"ancestor_record_offset":23180,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":565,"record_offset":38448,"parent_record_offset":38340,"ancestor_record_offset":23180,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":565,"record_offset":38176,"parent_record_offset":38148,"ancestor_record_offset":23180,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":38340,"parent_record_offset":38232,"ancestor_record_offset":23180,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":38204,"parent_record_offset":38176,"ancestor_record_offset":23180,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":565,"record_offset":29272,"parent_record_offset":29248,"ancestor_record_offset":23180,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":25164,"parent_record_offset":25140,"ancestor_record_offset":23180,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":565,"record_offset":25272,"parent_record_offset":25212,"ancestor_record_offset":23180,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":565,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":23180,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":565,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":23180,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":565,"record_offset":26860,"parent_record_offset":26836,"ancestor_record_offset":23180,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":32664,"parent_record_offset":32620,"ancestor_record_offset":23180,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":25140,"parent_record_offset":25012,"ancestor_record_offset":23180,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":33308,"parent_record_offset":33284,"ancestor_record_offset":23180,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":25320,"parent_record_offset":25296,"ancestor_record_offset":23180,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":565,"record_offset":26604,"parent_record_offset":26572,"ancestor_record_offset":23180,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":565,"record_offset":26628,"parent_record_offset":26604,"ancestor_record_offset":23180,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":565,"record_offset":24652,"parent_record_offset":24628,"ancestor_record_offset":23180,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":25892,"parent_record_offset":25864,"ancestor_record_offset":23180,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":33040,"parent_record_offset":33016,"ancestor_record_offset":23180,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":33064,"parent_record_offset":33040,"ancestor_record_offset":23180,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":25384,"parent_record_offset":25212,"ancestor_record_offset":23180,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":27232,"parent_record_offset":27200,"ancestor_record_offset":23180,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":24816,"parent_record_offset":24760,"ancestor_record_offset":23180,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":565,"record_offset":26256,"parent_record_offset":26232,"ancestor_record_offset":23180,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":565,"record_offset":26720,"parent_record_offset":26692,"ancestor_record_offset":23180,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":24396,"parent_record_offset":24360,"ancestor_record_offset":23180,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":565,"record_offset":29448,"parent_record_offset":29420,"ancestor_record_offset":23180,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":23180,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":28736,"parent_record_offset":28712,"ancestor_record_offset":23180,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":33756,"parent_record_offset":33732,"ancestor_record_offset":23180,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":29420,"parent_record_offset":29248,"ancestor_record_offset":23180,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":24516,"parent_record_offset":24484,"ancestor_record_offset":23180,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":565,"record_offset":33616,"parent_record_offset":33568,"ancestor_record_offset":23180,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":25744,"parent_record_offset":25656,"ancestor_record_offset":23180,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":565,"record_offset":26980,"parent_record_offset":26956,"ancestor_record_offset":23180,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":23672,"parent_record_offset":23644,"ancestor_record_offset":23180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":26660,"parent_record_offset":26572,"ancestor_record_offset":23180,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":26956,"parent_record_offset":26932,"ancestor_record_offset":23180,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":26496,"parent_record_offset":26100,"ancestor_record_offset":23180,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":24604,"parent_record_offset":24484,"ancestor_record_offset":23180,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":25952,"parent_record_offset":25820,"ancestor_record_offset":23180,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":25244,"parent_record_offset":25212,"ancestor_record_offset":23180,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":26932,"parent_record_offset":26908,"ancestor_record_offset":23180,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":565,"record_offset":29352,"parent_record_offset":29324,"ancestor_record_offset":23180,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":23180,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":23180,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":565,"record_offset":30008,"parent_record_offset":29848,"ancestor_record_offset":23180,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":29556,"parent_record_offset":29532,"ancestor_record_offset":23180,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":565,"record_offset":29584,"parent_record_offset":29556,"ancestor_record_offset":23180,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":33364,"parent_record_offset":32620,"ancestor_record_offset":23180,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":23180,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":23180,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":29296,"parent_record_offset":29272,"ancestor_record_offset":23180,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":25212,"parent_record_offset":25188,"ancestor_record_offset":23180,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":565,"record_offset":29324,"parent_record_offset":29296,"ancestor_record_offset":23180,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":38636,"parent_record_offset":38604,"ancestor_record_offset":23180,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":23180,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":565,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":23180,"inlinee":15545,"source_line":1017,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":565,"record_offset":29508,"parent_record_offset":29484,"ancestor_record_offset":23180,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33480,"parent_record_offset":33452,"ancestor_record_offset":23180,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23732,"parent_record_offset":23696,"ancestor_record_offset":23180,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33672,"parent_record_offset":33648,"ancestor_record_offset":23180,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33504,"parent_record_offset":33480,"ancestor_record_offset":23180,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":23180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":23180,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":565,"record_offset":23788,"parent_record_offset":23696,"ancestor_record_offset":23180,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":23180,"inlinee":35534,"source_line":511,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":36844,"parent_record_offset":36820,"ancestor_record_offset":23180,"inlinee":35539,"source_line":585,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":565,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":23180,"inlinee":35540,"source_line":590,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":26200,"parent_record_offset":26172,"ancestor_record_offset":23180,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":26232,"parent_record_offset":26200,"ancestor_record_offset":23180,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":565,"record_offset":26172,"parent_record_offset":26132,"ancestor_record_offset":23180,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":565,"record_offset":25688,"parent_record_offset":25656,"ancestor_record_offset":23180,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38772,"parent_record_offset":38748,"ancestor_record_offset":23180,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38796,"parent_record_offset":38772,"ancestor_record_offset":23180,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":25044,"parent_record_offset":25012,"ancestor_record_offset":23180,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":25520,"parent_record_offset":25484,"ancestor_record_offset":23180,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":26052,"parent_record_offset":26028,"ancestor_record_offset":23180,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":25012,"parent_record_offset":24984,"ancestor_record_offset":23180,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":25344,"parent_record_offset":25320,"ancestor_record_offset":23180,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":565,"record_offset":38824,"parent_record_offset":38796,"ancestor_record_offset":23180,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":26352,"parent_record_offset":26132,"ancestor_record_offset":23180,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25656,"parent_record_offset":25628,"ancestor_record_offset":23180,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":26132,"parent_record_offset":26100,"ancestor_record_offset":23180,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25820,"parent_record_offset":25784,"ancestor_record_offset":23180,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25784,"parent_record_offset":25556,"ancestor_record_offset":23180,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25600,"parent_record_offset":25556,"ancestor_record_offset":23180,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25072,"parent_record_offset":25044,"ancestor_record_offset":23180,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":31764,"parent_record_offset":31732,"ancestor_record_offset":23180,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":31676,"parent_record_offset":31652,"ancestor_record_offset":23180,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":31732,"parent_record_offset":31676,"ancestor_record_offset":23180,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":31704,"parent_record_offset":31676,"ancestor_record_offset":23180,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25628,"parent_record_offset":25600,"ancestor_record_offset":23180,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25864,"parent_record_offset":25820,"ancestor_record_offset":23180,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":31652,"parent_record_offset":31628,"ancestor_record_offset":23180,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":25556,"parent_record_offset":25520,"ancestor_record_offset":23180,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":26100,"parent_record_offset":26076,"ancestor_record_offset":23180,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":32432,"parent_record_offset":32408,"ancestor_record_offset":23180,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":565,"record_offset":26076,"parent_record_offset":26052,"ancestor_record_offset":23180,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":24984,"parent_record_offset":24956,"ancestor_record_offset":23180,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":25484,"parent_record_offset":25448,"ancestor_record_offset":23180,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":565,"record_offset":26028,"parent_record_offset":26004,"ancestor_record_offset":23180,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":565,"record_offset":38748,"parent_record_offset":38604,"ancestor_record_offset":23180,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":24928,"parent_record_offset":24224,"ancestor_record_offset":23180,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":24956,"parent_record_offset":24224,"ancestor_record_offset":23180,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":565,"record_offset":24224,"parent_record_offset":24184,"ancestor_record_offset":23180,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":24304,"parent_record_offset":24276,"ancestor_record_offset":23180,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":24564,"parent_record_offset":24540,"ancestor_record_offset":23180,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":565,"record_offset":24424,"parent_record_offset":24396,"ancestor_record_offset":23180,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":24760,"parent_record_offset":24360,"ancestor_record_offset":23180,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":565,"record_offset":24276,"parent_record_offset":24224,"ancestor_record_offset":23180,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":23180,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":565,"record_offset":24484,"parent_record_offset":24452,"ancestor_record_offset":23180,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":23180,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":565,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":23180,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 475
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15926
// 首条位置: {"module":1822,"inlinee":88358}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1822,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":37240,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1822,"record_offset":37464,"parent_record_offset":37312,"ancestor_record_offset":37240,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1822,"record_offset":37576,"parent_record_offset":37548,"ancestor_record_offset":37240,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1822,"record_offset":37548,"parent_record_offset":37312,"ancestor_record_offset":37240,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1822,"record_offset":37404,"parent_record_offset":37380,"ancestor_record_offset":37240,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1822,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":37240,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1822,"record_offset":37512,"parent_record_offset":37488,"ancestor_record_offset":37240,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1822,"record_offset":37380,"parent_record_offset":37312,"ancestor_record_offset":37240,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1822,"record_offset":37352,"parent_record_offset":37312,"ancestor_record_offset":37240,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1822,"record_offset":37312,"parent_record_offset":37288,"ancestor_record_offset":37240,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1822,"record_offset":37288,"parent_record_offset":37264,"ancestor_record_offset":37240,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 494
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71AE
// 首条位置: {"module":508,"inlinee":29102}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":508,"record_offset":125452,"parent_record_offset":125428,"ancestor_record_offset":125428,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":508,"record_offset":125648,"parent_record_offset":125500,"ancestor_record_offset":125428,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":508,"record_offset":125760,"parent_record_offset":125732,"ancestor_record_offset":125428,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":508,"record_offset":125732,"parent_record_offset":125500,"ancestor_record_offset":125428,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":508,"record_offset":125588,"parent_record_offset":125564,"ancestor_record_offset":125428,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":125672,"parent_record_offset":125648,"ancestor_record_offset":125428,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":508,"record_offset":125696,"parent_record_offset":125672,"ancestor_record_offset":125428,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":125564,"parent_record_offset":125500,"ancestor_record_offset":125428,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":125540,"parent_record_offset":125500,"ancestor_record_offset":125428,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":125500,"parent_record_offset":125476,"ancestor_record_offset":125428,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":508,"record_offset":125476,"parent_record_offset":125452,"ancestor_record_offset":125428,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8ACE
// 首条位置: {"module":565,"inlinee":35534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 517
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71AF
// 首条位置: {"module":508,"inlinee":29103}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1643,"record_offset":295684,"parent_record_offset":295660,"ancestor_record_offset":295660,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":296332,"parent_record_offset":296308,"ancestor_record_offset":295660,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":295908,"parent_record_offset":295884,"ancestor_record_offset":295660,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":295992,"parent_record_offset":295968,"ancestor_record_offset":295660,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":296456,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":296140,"parent_record_offset":296112,"ancestor_record_offset":295660,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":296112,"parent_record_offset":296084,"ancestor_record_offset":295660,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":295932,"parent_record_offset":295908,"ancestor_record_offset":295660,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":296252,"parent_record_offset":296228,"ancestor_record_offset":295660,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":296404,"parent_record_offset":296380,"ancestor_record_offset":295660,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":296356,"parent_record_offset":296332,"ancestor_record_offset":295660,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":296052,"parent_record_offset":296024,"ancestor_record_offset":295660,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":296084,"parent_record_offset":296052,"ancestor_record_offset":295660,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":296024,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":295884,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":296484,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":295968,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1643,"record_offset":296608,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1643,"record_offset":296308,"parent_record_offset":296288,"ancestor_record_offset":295660,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":295740,"parent_record_offset":295712,"ancestor_record_offset":295660,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":296288,"parent_record_offset":296184,"ancestor_record_offset":295660,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":296676,"parent_record_offset":295740,"ancestor_record_offset":295660,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":296228,"parent_record_offset":296204,"ancestor_record_offset":295660,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":296576,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":296204,"parent_record_offset":296184,"ancestor_record_offset":295660,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":295800,"parent_record_offset":295772,"ancestor_record_offset":295660,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":295772,"parent_record_offset":295740,"ancestor_record_offset":295660,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":295828,"parent_record_offset":295800,"ancestor_record_offset":295660,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":296636,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":296184,"parent_record_offset":295828,"ancestor_record_offset":295660,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1643,"record_offset":295712,"parent_record_offset":295684,"ancestor_record_offset":295660,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1643,"record_offset":296380,"parent_record_offset":296356,"ancestor_record_offset":295660,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 520
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71A8
// 首条位置: {"module":508,"inlinee":29096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":508,"record_offset":125000,"parent_record_offset":124500,"ancestor_record_offset":124500,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":508,"record_offset":124540,"parent_record_offset":124500,"ancestor_record_offset":124500,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124788,"parent_record_offset":124756,"ancestor_record_offset":124500,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124840,"parent_record_offset":124816,"ancestor_record_offset":124500,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124816,"parent_record_offset":124788,"ancestor_record_offset":124500,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":508,"record_offset":125024,"parent_record_offset":125000,"ancestor_record_offset":124500,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":508,"record_offset":124948,"parent_record_offset":124920,"ancestor_record_offset":124500,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124712,"parent_record_offset":124676,"ancestor_record_offset":124500,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124756,"parent_record_offset":124712,"ancestor_record_offset":124500,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":508,"record_offset":124604,"parent_record_offset":124572,"ancestor_record_offset":124500,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":508,"record_offset":124920,"parent_record_offset":124604,"ancestor_record_offset":124500,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":508,"record_offset":124676,"parent_record_offset":124644,"ancestor_record_offset":124500,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":508,"record_offset":125092,"parent_record_offset":125068,"ancestor_record_offset":124500,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":508,"record_offset":124572,"parent_record_offset":124540,"ancestor_record_offset":124500,"inlinee":29098,"source_line":526,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":508,"record_offset":124644,"parent_record_offset":124604,"ancestor_record_offset":124500,"inlinee":29100,"source_line":529,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":508,"record_offset":125044,"parent_record_offset":125024,"ancestor_record_offset":124500,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":508,"record_offset":125068,"parent_record_offset":125044,"ancestor_record_offset":124500,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 526
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71AA
// 首条位置: {"module":508,"inlinee":29098}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":508,"record_offset":124604,"parent_record_offset":124572,"ancestor_record_offset":124572,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124788,"parent_record_offset":124756,"ancestor_record_offset":124572,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124840,"parent_record_offset":124816,"ancestor_record_offset":124572,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124816,"parent_record_offset":124788,"ancestor_record_offset":124572,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":508,"record_offset":124948,"parent_record_offset":124920,"ancestor_record_offset":124572,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124712,"parent_record_offset":124676,"ancestor_record_offset":124572,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124756,"parent_record_offset":124712,"ancestor_record_offset":124572,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":508,"record_offset":124920,"parent_record_offset":124604,"ancestor_record_offset":124572,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":508,"record_offset":124676,"parent_record_offset":124644,"ancestor_record_offset":124572,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod_hot_reload.rs
//     evidence={"module":508,"record_offset":124644,"parent_record_offset":124604,"ancestor_record_offset":124572,"inlinee":29100,"source_line":529,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 529
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71AC
// 首条位置: {"module":508,"inlinee":29100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":508,"record_offset":124676,"parent_record_offset":124644,"ancestor_record_offset":124644,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124788,"parent_record_offset":124756,"ancestor_record_offset":124644,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124840,"parent_record_offset":124816,"ancestor_record_offset":124644,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":508,"record_offset":124816,"parent_record_offset":124788,"ancestor_record_offset":124644,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124712,"parent_record_offset":124676,"ancestor_record_offset":124644,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":508,"record_offset":124756,"parent_record_offset":124712,"ancestor_record_offset":124644,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/00508.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 544
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1651,"record_offset":314224,"rva":24355424,"va":5393064544}
// 编译实例 RVAs: 0x173a260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e8d98395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e865231dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e822a20000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::ComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e875957eff","size":5,"soft_fail":false,"target_va":5384583168,"va":5393064582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1651,"record_offset":314576,"parent_record_offset":314224,"ancestor_record_offset":314224,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":314632,"parent_record_offset":314600,"ancestor_record_offset":314224,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":1651,"record_offset":314600,"parent_record_offset":314576,"ancestor_record_offset":314224,"inlinee":81751,"source_line":183,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$7
// 记录形式: compiled_procedure; instances=1
// 源码行: 544
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1651,"record_offset":311416,"rva":24355424,"va":5393064544}
// 编译实例 RVAs: 0x173a260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e8d98395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e865231dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_hot_reload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e822a20000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::ComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064544,"raw_hex":"e875957eff","size":5,"soft_fail":false,"target_va":5384583168,"va":5393064582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":311572,"parent_record_offset":311416,"ancestor_record_offset":311416,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 585
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AD3
// 首条位置: {"module":565,"inlinee":35539}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 590
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AD4
// 首条位置: {"module":565,"inlinee":35540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":38364,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":565,"record_offset":38412,"parent_record_offset":38388,"ancestor_record_offset":38364,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00565.jsonl.gz"}
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
// 完整结构及原始类型记录: mod_hot_reload.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::mod_hot_reload::ChangedModFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::startup::mod_hot_reload::ChangedModFile>", "constructor": "ref$", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> >", "constructor": "ref$", "arguments": ["ref$<country_core::startup::mod_hot_reload::ChangedModFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> > >", "constructor": "tuple$", "arguments": ["ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> > >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$2>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> > >", "country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::impl$0::equivalent<country_core::startup::mod_hot_reload::ChangedModFile,country_core::startup::mod_hot_reload::ChangedModFile>", "constructor": "hashbrown::impl$0::equivalent", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "country_core::startup::mod_hot_reload::ChangedModFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>", "constructor": "std::collections::hash::set::IntoIter", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::set::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::set::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >", "constructor": "hashbrown::set::impl$10::extend::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::mod_hot_reload::ChangedModFile>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::set::HashSet", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::Mutex", "arguments": ["std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$62::default<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "alloc::sync::impl$62::default", "arguments": ["std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "constructor": "tuple$", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>", "country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>", "country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>", "country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<str$>", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1>", "constructor": "hashbrown::set::impl$1::retain::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::retain<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1> >", "constructor": "hashbrown::map::HashMap::retain", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::startup::mod_hot_reload::ChangedModFile> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<country_core::startup::mod_hot_reload::ChangedModFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >", "tuple$<>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["country_core::startup::mod_hot_reload::ChangedModFile", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::startup::mod_hot_reload::ChangedModFile,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::mod_hot_reload::ModHotReload
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 68
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> >", "constructor": "core::option::Option", "arguments": ["bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >", "constructor": "enum2$", "arguments": ["core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>,tuple$<>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::after", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "tuple$<>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::system::commands::command::insert_resource::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::system::commands::command::remove_resource::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload> >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::insert_resource<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::system::commands::Commands::insert_resource", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$128::init_access<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics> >", "constructor": "bevy_ecs::system::system_param::impl$128::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "bool", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::schedule::config::impl$3::run_if", "arguments": ["alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "bool", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::system::commands::command::remove_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::startup::mod_hot_reload::ModHotReload>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "bool", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "tuple$<>", "void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),bool,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<country_core::startup::mod_hot_reload::ModHotReload>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["country_core::startup::mod_hot_reload::ModHotReload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::mod_hot_reload::WatchedModRoot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$4::equal_same_length<country_core::startup::mod_hot_reload::WatchedModRoot,country_core::startup::mod_hot_reload::WatchedModRoot>", "constructor": "core::slice::cmp::impl$4::equal_same_length", "arguments": ["country_core::startup::mod_hot_reload::WatchedModRoot", "country_core::startup::mod_hot_reload::WatchedModRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::mod_hot_reload::WatchedModRoot,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::mod_hot_reload::WatchedModRoot", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::mod_hot_reload::WatchedModRoot,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::mod_hot_reload::WatchedModRoot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["tg_mods::overlay::Mod", "country_core::startup::mod_hot_reload::WatchedModRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2>,country_core::startup::mod_hot_reload::WatchedModRoot>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2>", "country_core::startup::mod_hot_reload::WatchedModRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["country_core::startup::mod_hot_reload::WatchedModRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$>", "constructor": "core::result::Result", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot>", "constructor": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0", "arguments": ["country_core::startup::mod_hot_reload::WatchedModRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> >", "constructor": "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0", "arguments": ["tg_mods::overlay::Mod", "country_core::startup::mod_hot_reload::WatchedModRoot", "alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >", "country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> > >", "constructor": "core::iter::adapters::filter::filter_try_fold::closure_env$0", "arguments": ["tg_mods::overlay::Mod", "alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >", "country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1", "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["tg_mods::overlay::Mod", "alloc::alloc::Global", "alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>", "core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> > >", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod_hot_reload.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod_hot_reload.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 32; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 信息 ID: c7bfb1831d4a4cadfbb9b847
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0", "tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::classify
// 信息 ID: 7a0f5a4e027800eadd53ade9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2 @ 0xa333af -> 0xb31a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2c833 -> 0xb31a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::map::HashMap::retain<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1> > @ 0x17b2454 -> 0xb31a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::classify::closure$0
// 信息 ID: 83c54e49a2ef32b2ab103a20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::mod_hot_reload::classify", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::classify; module=565, inline_record=56172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::event_paths
// 信息 ID: 7ef5ab657d2c45c054c66956
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2 @ 0xa330b2 -> 0xb2c260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::impl$11::eq
// 信息 ID: 5e28273e32831e23e94fea9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal_same_length<country_core::startup::mod_hot_reload::WatchedModRoot,country_core::startup::mod_hot_reload::WatchedModRoot>; module=508, inline_record=129092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::impl$7::eq
// 信息 ID: 5641d755a13a3c3eb76efd9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::impl$0::equivalent<country_core::startup::mod_hot_reload::ChangedModFile,country_core::startup::mod_hot_reload::ChangedModFile>; module=508, inline_record=128316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::impl$9::hash
// 信息 ID: 4ca1457fdfd96314d0a4d8d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::startup::mod_hot_reload::ChangedModFile> >; module=1660, inline_record=37784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::is_replacement_watched
// 信息 ID: c38fb74f27dfc5cbc2ef81a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=7864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::is_replacement_watched::closure$0
// 信息 ID: 6ae51563cc10904c134dba71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::mod_hot_reload::is_replacement_watched", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=8060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::mod_hot_reload_apply
// 信息 ID: c50c94ed5df93a7f916220f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 30397cc9e0708a0cf8b3f159: void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> @ 0x92611c -> 0xb2c4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> > @ 0x192b7c2 -> 0xb2c4e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches
// 信息 ID: 54ca3bf4a0d661aaf3791a3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5667a1f1456d868a159bfb49: void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> @ 0x9265fe -> 0xb30990 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),tuple$<>,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)> > @ 0x195f68b -> 0xb30990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$0
// 信息 ID: 66d813c1273b4571e7c95a25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches; module=565, inline_record=49968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$1
// 信息 ID: 47eae5f0e205485feeefd9c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1 <- hashbrown::map::HashMap::retain<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] hashbrown::map::HashMap::retain<country_core::startup::mod_hot_reload::ChangedModFile,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<country_core::startup::mod_hot_reload::ChangedModFile,std::hash::random::RandomState,alloc::alloc::Global,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$1> >; module=1658, inline_record=411540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2
// 信息 ID: d3114fbd6a283b2693d00253
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2,tuple$<enum2$<notify::DebouncedEvent> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2 <- core::ptr::drop_in_place<country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2 <- hotwatch::GenericHotwatch::watch<notify::windows::ReadDirectoryChangesWatcher,ref$<std::path::PathBuf>,country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 3 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure_env$2,tuple$<enum2$<notify::DebouncedEvent> > > @ 0xa35f0a -> 0xa33070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2::closure$0
// 信息 ID: 1db22a973b8f249add56a135
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches::closure$2; module=534, inline_record=40844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter
// 信息 ID: 62e00de41fb76e66c43f1fba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=23180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$1
// 信息 ID: d62db96b2ede3bb4ff85de67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$1 <- alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<tg_mods::overlay::Mod,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<tg_mods::overlay::Mod,tuple$<> > > >; module=1822, inline_record=37240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$2
// 信息 ID: 0f20dbc04fb334300973f23f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$2 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> > >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::startup::mod_hot_reload::ChangedModFile> > >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$2>; module=508, inline_record=125428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$4
// 信息 ID: 2a627f175d2a3e8003a9a350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=31148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$5
// 信息 ID: 85fe3cb703c20b3956b80656
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$5 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$5>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$5>,alloc::alloc::Global>; module=1643, inline_record=295660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6
// 信息 ID: f78864bc759a546c9ae485af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::clutter_packs::ClutterPackObject,core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6 <- alloc::vec::into_iter::impl$6::try_fold<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,ref_mut$<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>; module=508, inline_record=124500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0
// 信息 ID: 55d6790c0baf8ec0d72dca58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>; module=508, inline_record=124572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$6::closure$0::closure$0
// 信息 ID: 31217dc066aee403f924aead
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>; module=508, inline_record=124644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$7
// 信息 ID: 08a6d1ddf8530a0c78ea7abc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterGlbImport>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> >,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterPackBuildRequest>,clutter_pack_pipe::build_clutter_packs::async_fn$0::closure_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<clutter_pack_pipe::ClutterMaterialEntry,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterial>,clutter_pack_pipe::build_clutter_packs::async_fn$0::closure_env$3<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7 <- clutter_pack_pipe::build_clutter_packs::async_fn$0::closure$1<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7 <- std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >,enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7 <- core::ptr::drop_in_place<enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$7> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 5 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] namespace_unique_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$8
// 信息 ID: 882277da54cff65661d427d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=36844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_clutter::closure$9
// 信息 ID: 4db45ee195b1f249499ae433
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=38364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_replacements
// 信息 ID: dc36d172df772f39fdd0ed69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=14896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_replacements::closure$1
// 信息 ID: 2d11b5360b3d31fdd238d4ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_replacements", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; module=1643, inline_record=668984; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; module=1643, inline_record=670276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::reload_changed_replacements::closure$2
// 信息 ID: b684d10ed9f5ac1bb69a15e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::mod_hot_reload::reload_changed_replacements", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; module=1643, inline_record=669252; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::startup::mod_hot_reload::ChangedModFile>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$1>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$2>,country_core::startup::mod_hot_reload::reload_changed_replacements::closure_env$3> >; module=1643, inline_record=670532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::watchable_mod_roots
// 信息 ID: 84219dd1fa3b8c72df3d8b7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::refresh_mod_hot_reload_watches; module=565, inline_record=46472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::watchable_mod_roots::closure$0
// 信息 ID: 6c86e33665b99d11407292fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::mod_hot_reload::watchable_mod_roots", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,std::path::PathBuf,tuple$<>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,tuple$<std::path::PathBuf,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<std::path::PathBuf,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,std::path::PathBuf,tuple$<>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,tuple$<std::path::PathBuf,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<std::path::PathBuf,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,std::path::PathBuf,tuple$<>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,tuple$<std::path::PathBuf,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<std::path::PathBuf,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,std::path::PathBuf,tuple$<>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,tuple$<std::path::PathBuf,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<std::path::PathBuf,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> > > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,std::path::PathBuf,tuple$<>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,tuple$<std::path::PathBuf,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<std::path::PathBuf,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$0> > > > > > > >; module=1822, inline_record=16960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::mod_hot_reload::watchable_mod_roots::closure$2
// 信息 ID: 56c6ca4804f9b618b2313979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::mod_hot_reload::watchable_mod_roots", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1>,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2>,country_core::startup::mod_hot_reload::WatchedModRoot>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2 <- alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$1,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::startup::mod_hot_reload::WatchedModRoot,alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> >,country_core::startup::mod_hot_reload::watchable_mod_roots::closure_env$2,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::startup::mod_hot_reload::WatchedModRoot> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::startup::mod_hot_reload::WatchedModRoot>,never$> > >; module=1822, inline_record=45776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
