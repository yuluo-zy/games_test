// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/screenshot.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 485b1487dd788f92ccc023e86f796fb9ef61e75d4fa4e9bf9177fabf5082b71a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 6, 81, 86, 107, 164, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466","version":"0.11.0"}]
// use tempfile; // namespace candidate
//   package identities: [{"name":"tempfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.25.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":55108,"rva":4976528,"va":5373685648}
// 编译实例 RVAs: 0x4bef90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cef00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3080389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373685648,"raw_hex":"e83bffd0ff","size":5,"soft_fail":false,"target_va":5370605312,"va":5373685696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cfb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3077245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373685648,"raw_hex":"e8830bd1ff","size":5,"soft_fail":false,"target_va":5370608448,"va":5373685688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140321090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1695522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373685648,"raw_hex":"e8de20e6ff","size":5,"soft_fail":false,"target_va":5371990160,"va":5373685677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":164,"record_offset":55232,"parent_record_offset":55108,"ancestor_record_offset":55108,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A39
// 首条位置: {"module":164,"inlinee":14905}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A2F
// 首条位置: {"module":164,"inlinee":14895}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":6476,"parent_record_offset":6416,"ancestor_record_offset":6416,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":6560,"parent_record_offset":6416,"ancestor_record_offset":6416,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6416,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":6416,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":6416,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7040,"parent_record_offset":6888,"ancestor_record_offset":6416,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":6416,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6888,"parent_record_offset":6852,"ancestor_record_offset":6416,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":6416,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6928,"parent_record_offset":6888,"ancestor_record_offset":6416,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":6416,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6736,"parent_record_offset":6700,"ancestor_record_offset":6416,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":6416,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":6416,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":6416,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":164,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":6416,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":6416,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":164,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":6416,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":6416,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":7168,"parent_record_offset":6764,"ancestor_record_offset":6416,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6416,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":6416,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":6416,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":6416,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":164,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":6416,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$1::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14A2
// 首条位置: {"module":6,"inlinee":5282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":6,"record_offset":1320848,"parent_record_offset":1320816,"ancestor_record_offset":1320816,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_direct] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":6,"record_offset":1320876,"parent_record_offset":1320816,"ancestor_record_offset":1320816,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":6,"record_offset":1320960,"parent_record_offset":1320932,"ancestor_record_offset":1320816,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":6,"record_offset":1320984,"parent_record_offset":1320960,"ancestor_record_offset":1320816,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":6,"record_offset":1320932,"parent_record_offset":1320904,"ancestor_record_offset":1320816,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":6,"record_offset":1320904,"parent_record_offset":1320876,"ancestor_record_offset":1320816,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00006.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C59
// 首条位置: {"module":164,"inlinee":11353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":23392,"parent_record_offset":23356,"ancestor_record_offset":23356,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":23492,"parent_record_offset":23468,"ancestor_record_offset":23356,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":23516,"parent_record_offset":23492,"ancestor_record_offset":23356,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":23540,"parent_record_offset":23516,"ancestor_record_offset":23356,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":23468,"parent_record_offset":23436,"ancestor_record_offset":23356,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":23436,"parent_record_offset":23392,"ancestor_record_offset":23356,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A2D
// 首条位置: {"module":164,"inlinee":14893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$31::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":10496,"parent_record_offset":10356,"ancestor_record_offset":10356,"inlinee":12245,"source_line":2239,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":10404,"parent_record_offset":10356,"ancestor_record_offset":10356,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":10460,"parent_record_offset":10356,"ancestor_record_offset":10356,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":10356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":10356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":10356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":10656,"parent_record_offset":10524,"ancestor_record_offset":10356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":10524,"parent_record_offset":10496,"ancestor_record_offset":10356,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":10556,"parent_record_offset":10524,"ancestor_record_offset":10356,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":10620,"parent_record_offset":10584,"ancestor_record_offset":10356,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":10356,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":10584,"parent_record_offset":10556,"ancestor_record_offset":10356,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":16388,"rva":4957600,"va":5373666720}
// 编译实例 RVAs: 0x4ba5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11124884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e894c0a900","size":5,"soft_fail":false,"target_va":5384793264,"va":5373668375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t10988390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e866aba700","size":5,"soft_fail":false,"target_va":5384656240,"va":5373667845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t2010530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8a2ad1e00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373668153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t2010742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e876ae1e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373667957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4945951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8e187b4ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373668138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4447534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8d222bcff","size":5,"soft_fail":false,"target_va":5369226528,"va":5373674057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a19e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4298671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e85168beff","size":5,"soft_fail":false,"target_va":5369371104,"va":5373669770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ae2c0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4250108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e80426bfff","size":5,"soft_fail":false,"target_va":5369422528,"va":5373672631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b4630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4224396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8748abfff","size":5,"soft_fail":false,"target_va":5369447984,"va":5373672375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400be9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4184914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8ae24c0ff","size":5,"soft_fail":false,"target_va":5369489840,"va":5373674749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1840705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8bfe9e3ff","size":5,"soft_fail":false,"target_va":5371826816,"va":5373667516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140320ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1684460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8144ce6ff","size":5,"soft_fail":false,"target_va":5371989744,"va":5373674199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=3
//     target_aliases=49 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1206783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e80196edff","size":5,"soft_fail":false,"target_va":5372465520,"va":5373672298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-540879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e831bff7ff","size":5,"soft_fail":false,"target_va":5373130976,"va":5373671850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e2690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1208140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e84c6f1200","size":5,"soft_fail":false,"target_va":5374879376,"va":5373671231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7937730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8c21e7900","size":5,"soft_fail":false,"target_va":5381607664,"va":5373669929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90b20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8214851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e843597d00","size":5,"soft_fail":false,"target_va":5381884704,"va":5373669848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91910
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8220917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8f5707d00","size":5,"soft_fail":false,"target_va":5381888272,"va":5373667350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92440
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8221369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8b9727d00","size":5,"soft_fail":false,"target_va":5381891136,"va":5373669762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42682469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e865488b02","size":5,"soft_fail":false,"target_va":5416350720,"va":5373668246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42680223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e89f3f8b02","size":5,"soft_fail":false,"target_va":5416351840,"va":5373671612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fbe0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42684289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8814f8b02","size":5,"soft_fail":false,"target_va":5416352736,"va":5373668442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42681114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e81a438b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5373671921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43388472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8380e9602","size":5,"soft_fail":false,"target_va":5417057056,"va":5373668579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t42948704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e860588f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5373672060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11126927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e88fc8a900","size":5,"soft_fail":false,"target_va":5384794864,"va":5373667932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42952487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e827678f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5373668302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8ef250000","size":5,"soft_fail":false,"target_va":5373684368,"va":5373674652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4737383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e867494800","size":5,"soft_fail":false,"target_va":5378404688,"va":5373667300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::consume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5715982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e80e385700","size":5,"soft_fail":false,"target_va":5379387408,"va":5373671421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::peek
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5716105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e889385700","size":5,"soft_fail":false,"target_va":5379387360,"va":5373671250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7938496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8c0217900","size":5,"soft_fail":false,"target_va":5381607616,"va":5373669115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42904946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e872ad8e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373671881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42904184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e878aa8e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373671955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42900832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8609d8e02","size":5,"soft_fail":false,"target_va":5416572816,"va":5373671979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42902907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e87ba58e02","size":5,"soft_fail":false,"target_va":5416574944,"va":5373672032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10317942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e876709d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373666901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10318368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e820729d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373666987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9471757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e80d879000","size":5,"soft_fail":false,"target_va":5383139568,"va":5373667806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9632182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373666720,"raw_hex":"e8b6f99200","size":5,"soft_fail":false,"target_va":5383300064,"va":5373667877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":164,"record_offset":19948,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":24812,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":21840,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":21316,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":21560,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":16620,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":21928,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":29696,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":29752,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":164,"record_offset":19680,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":24624,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":164,"record_offset":17904,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":25664,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":164,"record_offset":23016,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":164,"record_offset":29284,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":164,"record_offset":20316,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":18664,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":20032,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":29540,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":21344,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":19920,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":22324,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":23356,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":11353,"source_line":96,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::add_captured_screenshots_to_steam_library
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":19076,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":14901,"source_line":460,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::consume_steam_screenshot_request
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":16648,"parent_record_offset":16388,"ancestor_record_offset":16388,"inlinee":14896,"source_line":441,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":21680,"parent_record_offset":21656,"ancestor_record_offset":16388,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":21704,"parent_record_offset":21680,"ancestor_record_offset":16388,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":16388,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":20232,"parent_record_offset":20208,"ancestor_record_offset":16388,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":21728,"parent_record_offset":21704,"ancestor_record_offset":16388,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":20256,"parent_record_offset":20232,"ancestor_record_offset":16388,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":21656,"parent_record_offset":21624,"ancestor_record_offset":16388,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":20184,"parent_record_offset":20128,"ancestor_record_offset":16388,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":21624,"parent_record_offset":21596,"ancestor_record_offset":16388,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":23392,"parent_record_offset":23356,"ancestor_record_offset":16388,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":21596,"parent_record_offset":21560,"ancestor_record_offset":16388,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$6::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":21780,"parent_record_offset":21560,"ancestor_record_offset":16388,"inlinee":7479,"source_line":1212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":164,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":16388,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":16388,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":29928,"parent_record_offset":29900,"ancestor_record_offset":16388,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22588,"parent_record_offset":22560,"ancestor_record_offset":16388,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":30012,"parent_record_offset":29988,"ancestor_record_offset":16388,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":16388,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":16388,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":29988,"parent_record_offset":29956,"ancestor_record_offset":16388,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22068,"parent_record_offset":22044,"ancestor_record_offset":16388,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22044,"parent_record_offset":22020,"ancestor_record_offset":16388,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":29956,"parent_record_offset":29928,"ancestor_record_offset":16388,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22652,"parent_record_offset":22616,"ancestor_record_offset":16388,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22616,"parent_record_offset":22588,"ancestor_record_offset":16388,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":16388,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":22692,"parent_record_offset":22652,"ancestor_record_offset":16388,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":22528,"parent_record_offset":22464,"ancestor_record_offset":16388,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":26092,"parent_record_offset":26056,"ancestor_record_offset":16388,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":16728,"parent_record_offset":16648,"ancestor_record_offset":16388,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":17520,"parent_record_offset":17496,"ancestor_record_offset":16388,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":21996,"parent_record_offset":21956,"ancestor_record_offset":16388,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":27972,"parent_record_offset":27944,"ancestor_record_offset":16388,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":16388,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":29852,"parent_record_offset":29816,"ancestor_record_offset":16388,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":21956,"parent_record_offset":21928,"ancestor_record_offset":16388,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":29900,"parent_record_offset":29852,"ancestor_record_offset":16388,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":17440,"parent_record_offset":17352,"ancestor_record_offset":16388,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":17352,"parent_record_offset":16648,"ancestor_record_offset":16388,"inlinee":8600,"source_line":2197,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":22560,"parent_record_offset":22528,"ancestor_record_offset":16388,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":22436,"parent_record_offset":22408,"ancestor_record_offset":16388,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":18268,"parent_record_offset":18008,"ancestor_record_offset":16388,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":164,"record_offset":30232,"parent_record_offset":29852,"ancestor_record_offset":16388,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":164,"record_offset":29816,"parent_record_offset":29752,"ancestor_record_offset":16388,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":164,"record_offset":22408,"parent_record_offset":22380,"ancestor_record_offset":16388,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":164,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":16388,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":18008,"parent_record_offset":17976,"ancestor_record_offset":16388,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":18720,"parent_record_offset":18688,"ancestor_record_offset":16388,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":24656,"parent_record_offset":24624,"ancestor_record_offset":16388,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":18748,"parent_record_offset":18720,"ancestor_record_offset":16388,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":164,"record_offset":17944,"parent_record_offset":17904,"ancestor_record_offset":16388,"inlinee":9341,"source_line":79,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":164,"record_offset":17976,"parent_record_offset":17944,"ancestor_record_offset":16388,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":164,"record_offset":27688,"parent_record_offset":27644,"ancestor_record_offset":16388,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":16388,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":16388,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":164,"record_offset":22764,"parent_record_offset":22740,"ancestor_record_offset":16388,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":164,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":16388,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":164,"record_offset":24688,"parent_record_offset":24656,"ancestor_record_offset":16388,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":164,"record_offset":18780,"parent_record_offset":18748,"ancestor_record_offset":16388,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":164,"record_offset":18884,"parent_record_offset":18860,"ancestor_record_offset":16388,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":164,"record_offset":29788,"parent_record_offset":29752,"ancestor_record_offset":16388,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":19012,"parent_record_offset":18688,"ancestor_record_offset":16388,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":30176,"parent_record_offset":29852,"ancestor_record_offset":16388,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":16388,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":16388,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":164,"record_offset":21804,"parent_record_offset":21780,"ancestor_record_offset":16388,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":16388,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":18220,"parent_record_offset":18188,"ancestor_record_offset":16388,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":18068,"parent_record_offset":18008,"ancestor_record_offset":16388,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":24716,"parent_record_offset":24688,"ancestor_record_offset":16388,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":16388,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":19972,"parent_record_offset":19948,"ancestor_record_offset":16388,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":28900,"parent_record_offset":28780,"ancestor_record_offset":16388,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":18860,"parent_record_offset":18780,"ancestor_record_offset":16388,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":24776,"parent_record_offset":24656,"ancestor_record_offset":16388,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":22932,"parent_record_offset":22528,"ancestor_record_offset":16388,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17408,"parent_record_offset":17352,"ancestor_record_offset":16388,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":16388,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17104,"parent_record_offset":16648,"ancestor_record_offset":16388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":164,"record_offset":18556,"parent_record_offset":18532,"ancestor_record_offset":16388,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":16388,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":26124,"parent_record_offset":26092,"ancestor_record_offset":16388,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":17036,"parent_record_offset":17008,"ancestor_record_offset":16388,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17380,"parent_record_offset":17352,"ancestor_record_offset":16388,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":22108,"parent_record_offset":21956,"ancestor_record_offset":16388,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":18124,"parent_record_offset":18008,"ancestor_record_offset":16388,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":18508,"parent_record_offset":18008,"ancestor_record_offset":16388,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":18188,"parent_record_offset":18156,"ancestor_record_offset":16388,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":18156,"parent_record_offset":18124,"ancestor_record_offset":16388,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":164,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":16388,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":18936,"parent_record_offset":18884,"ancestor_record_offset":16388,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":17008,"parent_record_offset":16972,"ancestor_record_offset":16388,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::position
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":16972,"parent_record_offset":16648,"ancestor_record_offset":16388,"inlinee":14897,"source_line":377,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":32000,"parent_record_offset":31936,"ancestor_record_offset":16388,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":29604,"parent_record_offset":29568,"ancestor_record_offset":16388,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17284,"parent_record_offset":17160,"ancestor_record_offset":16388,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":16820,"parent_record_offset":16784,"ancestor_record_offset":16388,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":21868,"parent_record_offset":21840,"ancestor_record_offset":16388,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":16388,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":16388,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17308,"parent_record_offset":17284,"ancestor_record_offset":16388,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":16388,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":16388,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17612,"parent_record_offset":17520,"ancestor_record_offset":16388,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":31908,"parent_record_offset":31880,"ancestor_record_offset":16388,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":16756,"parent_record_offset":16648,"ancestor_record_offset":16388,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$27::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":19372,"parent_record_offset":19076,"ancestor_record_offset":16388,"inlinee":14903,"source_line":1822,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":17132,"parent_record_offset":17104,"ancestor_record_offset":16388,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":29568,"parent_record_offset":29540,"ancestor_record_offset":16388,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":31936,"parent_record_offset":31908,"ancestor_record_offset":16388,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":16876,"parent_record_offset":16784,"ancestor_record_offset":16388,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":17196,"parent_record_offset":17160,"ancestor_record_offset":16388,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":16388,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":17160,"parent_record_offset":17132,"ancestor_record_offset":16388,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":16388,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":17224,"parent_record_offset":17196,"ancestor_record_offset":16388,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":16388,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":22352,"parent_record_offset":22324,"ancestor_record_offset":16388,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":16388,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":164,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":16388,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":164,"record_offset":22380,"parent_record_offset":22352,"ancestor_record_offset":16388,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":17072,"parent_record_offset":16972,"ancestor_record_offset":16388,"inlinee":14899,"source_line":450,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::gather_screenshot_requests::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":19036,"parent_record_offset":19012,"ancestor_record_offset":16388,"inlinee":14900,"source_line":191,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A34
// 首条位置: {"module":164,"inlinee":14900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C56
// 首条位置: {"module":86,"inlinee":11350}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":86,"record_offset":60036,"parent_record_offset":59984,"ancestor_record_offset":59984,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":86,"record_offset":60624,"parent_record_offset":59984,"ancestor_record_offset":59984,"inlinee":11351,"source_line":1943,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":86,"record_offset":60684,"parent_record_offset":59984,"ancestor_record_offset":59984,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":86,"record_offset":60220,"parent_record_offset":59984,"ancestor_record_offset":59984,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":86,"record_offset":60064,"parent_record_offset":59984,"ancestor_record_offset":59984,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":60496,"parent_record_offset":60472,"ancestor_record_offset":59984,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":60520,"parent_record_offset":60496,"ancestor_record_offset":59984,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":60544,"parent_record_offset":60520,"ancestor_record_offset":59984,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":86,"record_offset":60472,"parent_record_offset":60412,"ancestor_record_offset":59984,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":86,"record_offset":61132,"parent_record_offset":61104,"ancestor_record_offset":59984,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":86,"record_offset":60440,"parent_record_offset":60412,"ancestor_record_offset":59984,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":86,"record_offset":60412,"parent_record_offset":60388,"ancestor_record_offset":59984,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":86,"record_offset":60388,"parent_record_offset":60368,"ancestor_record_offset":59984,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":86,"record_offset":61772,"parent_record_offset":61736,"ancestor_record_offset":59984,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":86,"record_offset":61736,"parent_record_offset":61712,"ancestor_record_offset":59984,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":86,"record_offset":61160,"parent_record_offset":61132,"ancestor_record_offset":59984,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":86,"record_offset":61224,"parent_record_offset":61188,"ancestor_record_offset":59984,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":86,"record_offset":60648,"parent_record_offset":60624,"ancestor_record_offset":59984,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":86,"record_offset":61188,"parent_record_offset":61160,"ancestor_record_offset":59984,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":86,"record_offset":60248,"parent_record_offset":60220,"ancestor_record_offset":59984,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":86,"record_offset":61804,"parent_record_offset":61772,"ancestor_record_offset":59984,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":61644,"parent_record_offset":61580,"ancestor_record_offset":59984,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":60128,"parent_record_offset":60092,"ancestor_record_offset":59984,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":61668,"parent_record_offset":61644,"ancestor_record_offset":59984,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":60152,"parent_record_offset":60128,"ancestor_record_offset":59984,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":61832,"parent_record_offset":61772,"ancestor_record_offset":59984,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":86,"record_offset":61864,"parent_record_offset":61736,"ancestor_record_offset":59984,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":86,"record_offset":61552,"parent_record_offset":61524,"ancestor_record_offset":59984,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":86,"record_offset":60092,"parent_record_offset":60064,"ancestor_record_offset":59984,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":86,"record_offset":61580,"parent_record_offset":61552,"ancestor_record_offset":59984,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":86,"record_offset":60184,"parent_record_offset":60092,"ancestor_record_offset":59984,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":86,"record_offset":61616,"parent_record_offset":61580,"ancestor_record_offset":59984,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00086.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A30
// 首条位置: {"module":164,"inlinee":14896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":16728,"parent_record_offset":16648,"ancestor_record_offset":16648,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":17352,"parent_record_offset":16648,"ancestor_record_offset":16648,"inlinee":8600,"source_line":2197,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17104,"parent_record_offset":16648,"ancestor_record_offset":16648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::position
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":16972,"parent_record_offset":16648,"ancestor_record_offset":16648,"inlinee":14897,"source_line":377,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":16756,"parent_record_offset":16648,"ancestor_record_offset":16648,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":17520,"parent_record_offset":17496,"ancestor_record_offset":16648,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":17440,"parent_record_offset":17352,"ancestor_record_offset":16648,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17408,"parent_record_offset":17352,"ancestor_record_offset":16648,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17496,"parent_record_offset":17472,"ancestor_record_offset":16648,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":17036,"parent_record_offset":17008,"ancestor_record_offset":16648,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":17380,"parent_record_offset":17352,"ancestor_record_offset":16648,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":17008,"parent_record_offset":16972,"ancestor_record_offset":16648,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17284,"parent_record_offset":17160,"ancestor_record_offset":16648,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":16820,"parent_record_offset":16784,"ancestor_record_offset":16648,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":16648,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17308,"parent_record_offset":17284,"ancestor_record_offset":16648,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":16648,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":16648,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":17612,"parent_record_offset":17520,"ancestor_record_offset":16648,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":17132,"parent_record_offset":17104,"ancestor_record_offset":16648,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":16876,"parent_record_offset":16784,"ancestor_record_offset":16648,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":17196,"parent_record_offset":17160,"ancestor_record_offset":16648,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":16648,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":17160,"parent_record_offset":17132,"ancestor_record_offset":16648,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":16648,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":17224,"parent_record_offset":17196,"ancestor_record_offset":16648,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":17072,"parent_record_offset":16972,"ancestor_record_offset":16648,"inlinee":14899,"source_line":450,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A33
// 首条位置: {"module":164,"inlinee":14899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 460
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A35
// 首条位置: {"module":164,"inlinee":14901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":19128,"parent_record_offset":19076,"ancestor_record_offset":19076,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":19428,"parent_record_offset":19076,"ancestor_record_offset":19076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":19156,"parent_record_offset":19076,"ancestor_record_offset":19076,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$27::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":19372,"parent_record_offset":19076,"ancestor_record_offset":19076,"inlinee":14903,"source_line":1822,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":19076,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":19608,"parent_record_offset":19484,"ancestor_record_offset":19076,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":19220,"parent_record_offset":19184,"ancestor_record_offset":19076,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":19076,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":19076,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":164,"record_offset":19456,"parent_record_offset":19428,"ancestor_record_offset":19076,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":19276,"parent_record_offset":19184,"ancestor_record_offset":19076,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":19520,"parent_record_offset":19484,"ancestor_record_offset":19076,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":19184,"parent_record_offset":19156,"ancestor_record_offset":19076,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":164,"record_offset":19484,"parent_record_offset":19456,"ancestor_record_offset":19076,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":19304,"parent_record_offset":19276,"ancestor_record_offset":19076,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":19548,"parent_record_offset":19520,"ancestor_record_offset":19076,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 468
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":81,"record_offset":6608,"rva":3112064,"va":5371821184}
// 编译实例 RVAs: 0x2f7c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::is_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12835176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e868d9c300","size":5,"soft_fail":false,"target_va":5384656800,"va":5371821619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t12787883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8ab20c300","size":5,"soft_fail":false,"target_va":5384609408,"va":5371821520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3857120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8e0da3a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5371821579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2573529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e827bbd8ff","size":5,"soft_fail":false,"target_va":5369248944,"va":5371822468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2517029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8db97d9ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5371822608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4007406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8ee253d00","size":5,"soft_fail":false,"target_va":5375829056,"va":5371821645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b240
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45233052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e89c33b202","size":5,"soft_fail":false,"target_va":5417054784,"va":5371821727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45233253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e86534b202","size":5,"soft_fail":false,"target_va":5417054912,"va":5371821654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10175618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e882449b00","size":5,"soft_fail":false,"target_va":5381997408,"va":5371821785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t44799338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e86a95ab02","size":5,"soft_fail":false,"target_va":5416621600,"va":5371822257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t44754601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8a9e6aa02","size":5,"soft_fail":false,"target_va":5416576832,"va":5371822226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t44753896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8e8e3aa02","size":5,"soft_fail":false,"target_va":5416576144,"va":5371822243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12163591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8079ab900","size":5,"soft_fail":false,"target_va":5383984848,"va":5371821252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12164039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e8c79bb900","size":5,"soft_fail":false,"target_va":5383985360,"va":5371821316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] steamworks::screenshots::path_to_absolute_cstring
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     evidence={"assembly":"\tcallq\t10174834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371821184,"raw_hex":"e872419b00","size":5,"soft_fail":false,"target_va":5381996528,"va":5371821689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371821184,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371821555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":81,"record_offset":6808,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":81,"record_offset":7324,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":8020,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":6992,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":8048,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":81,"record_offset":7052,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":81,"record_offset":6836,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":81,"record_offset":7380,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":81,"record_offset":7240,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::screenshots
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":81,"record_offset":8084,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":10717,"source_line":442,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] steamworks::screenshots::Screenshots::add_screenshot_to_library
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/screenshots.rs
//     evidence={"module":81,"record_offset":8240,"parent_record_offset":6608,"ancestor_record_offset":6608,"inlinee":5870,"source_line":57,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":6608,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":6608,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":6608,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":7652,"parent_record_offset":7520,"ancestor_record_offset":6608,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":81,"record_offset":7520,"parent_record_offset":7492,"ancestor_record_offset":6608,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":81,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":6608,"inlinee":4646,"source_line":700,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":6608,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":9640,"parent_record_offset":9616,"ancestor_record_offset":6608,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":6608,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":81,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":6608,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":81,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":6608,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":81,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":6608,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":81,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":6608,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":81,"record_offset":7616,"parent_record_offset":7580,"ancestor_record_offset":6608,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":81,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6608,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":81,"record_offset":7580,"parent_record_offset":7552,"ancestor_record_offset":6608,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":6608,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":8276,"parent_record_offset":8240,"ancestor_record_offset":6608,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":81,"record_offset":8304,"parent_record_offset":8240,"ancestor_record_offset":6608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":81,"record_offset":8172,"parent_record_offset":8140,"ancestor_record_offset":6608,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":7172,"parent_record_offset":7108,"ancestor_record_offset":6608,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":6900,"parent_record_offset":6864,"ancestor_record_offset":6608,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":6608,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":8200,"parent_record_offset":8140,"ancestor_record_offset":6608,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":6608,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":6608,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":6608,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":8680,"parent_record_offset":8620,"ancestor_record_offset":6608,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":81,"record_offset":8712,"parent_record_offset":8584,"ancestor_record_offset":6608,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":81,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":6608,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":81,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":6608,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":81,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":6608,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":81,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":6608,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":81,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":6608,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":81,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":6608,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":81,"record_offset":6956,"parent_record_offset":6864,"ancestor_record_offset":6608,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":81,"record_offset":7144,"parent_record_offset":7108,"ancestor_record_offset":6608,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::next_requested_screenshot_stage
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 505
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3054
// 首条位置: {"module":107,"inlinee":12372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":61896,"parent_record_offset":61592,"ancestor_record_offset":61592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":107,"record_offset":61804,"parent_record_offset":61592,"ancestor_record_offset":61592,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":107,"record_offset":61648,"parent_record_offset":61592,"ancestor_record_offset":61592,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":61864,"parent_record_offset":61804,"ancestor_record_offset":61592,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":61836,"parent_record_offset":61804,"ancestor_record_offset":61592,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":62000,"parent_record_offset":61944,"ancestor_record_offset":61592,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":61712,"parent_record_offset":61676,"ancestor_record_offset":61592,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":62024,"parent_record_offset":62000,"ancestor_record_offset":61592,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":61736,"parent_record_offset":61712,"ancestor_record_offset":61592,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":107,"record_offset":61920,"parent_record_offset":61896,"ancestor_record_offset":61592,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":107,"record_offset":61676,"parent_record_offset":61648,"ancestor_record_offset":61592,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":107,"record_offset":61944,"parent_record_offset":61920,"ancestor_record_offset":61592,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":107,"record_offset":61768,"parent_record_offset":61676,"ancestor_record_offset":61592,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":107,"record_offset":61972,"parent_record_offset":61944,"ancestor_record_offset":61592,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::handle_screenshot_opportunity
// 记录形式: compiled_procedure; instances=1
// 源码行: 521
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":34444,"rva":4965648,"va":5373674768}
// 编译实例 RVAs: 0x4bc510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::export
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9815244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8ccc49500","size":5,"soft_fail":false,"target_va":5383492416,"va":5373677167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9815405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e86dc59500","size":5,"soft_fail":false,"target_va":5383492272,"va":5373676862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::export
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9816192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e880c89500","size":5,"soft_fail":false,"target_va":5383493744,"va":5373677547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9817981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e87dcf9500","size":5,"soft_fail":false,"target_va":5383493440,"va":5373675454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9796950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8567d9500","size":5,"soft_fail":false,"target_va":5383472768,"va":5373675813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4372959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e82146bdff","size":5,"soft_fail":false,"target_va":5369305584,"va":5373678538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f150
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4316505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8a722beff","size":5,"soft_fail":false,"target_va":5369360720,"va":5373677220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a19e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4306539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e89549beff","size":5,"soft_fail":false,"target_va":5369371104,"va":5373677638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d4c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1148125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8dd841100","size":5,"soft_fail":false,"target_va":5374823520,"va":5373675390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68700
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42644805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e845b58a02","size":5,"soft_fail":false,"target_va":5416322816,"va":5373678006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43380793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e839f09502","size":5,"soft_fail":false,"target_va":5417057056,"va":5373676258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43382424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e898f69502","size":5,"soft_fail":false,"target_va":5417057904,"va":5373675475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43382499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8e3f69502","size":5,"soft_fail":false,"target_va":5417057984,"va":5373675480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42943418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8ba438f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5373678177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e829150000","size":5,"soft_fail":false,"target_va":5373684368,"va":5373678946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42899011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e843968e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373677816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42898300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e87c938e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373677839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10309966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e84e519d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373674877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10307677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e85d489d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373677678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9811080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e888b49500","size":5,"soft_fail":false,"target_va":5383487824,"va":5373676739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_mapped_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10045273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e859479900","size":5,"soft_fail":false,"target_va":5383722048,"va":5373676770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9872400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e810a49600","size":5,"soft_fail":false,"target_va":5383548144,"va":5373675739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-283674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8e6abfbff","size":5,"soft_fail":false,"target_va":5373392400,"va":5373676069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9860343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8f7749600","size":5,"soft_fail":false,"target_va":5383536672,"va":5373676324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9863133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e8dd7f9600","size":5,"soft_fail":false,"target_va":5383539152,"va":5373676014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-787918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373674768,"raw_hex":"e832faf3ff","size":5,"soft_fail":false,"target_va":5372888064,"va":5373675977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":36836,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":34816,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":34844,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::product
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":164,"record_offset":36068,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14913,"source_line":3686,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":36592,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":35136,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":37556,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":34956,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":34660,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":37020,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::create_texture
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":164,"record_offset":35108,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14907,"source_line":881,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::storage
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":164,"record_offset":35080,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14906,"source_line":517,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35972,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::export
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":164,"record_offset":36536,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":13028,"source_line":105,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::import_mut_with_access_type
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":164,"record_offset":35172,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":12379,"source_line":1321,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":36040,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":164,"record_offset":35200,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14908,"source_line":541,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":164,"record_offset":35228,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":35048,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14905,"source_line":53,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::progressive_blur_downsample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":35284,"parent_record_offset":34444,"ancestor_record_offset":34444,"inlinee":14909,"source_line":892,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":164,"record_offset":36140,"parent_record_offset":36116,"ancestor_record_offset":34444,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":164,"record_offset":36164,"parent_record_offset":36140,"ancestor_record_offset":34444,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":164,"record_offset":36188,"parent_record_offset":36164,"ancestor_record_offset":34444,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":164,"record_offset":36236,"parent_record_offset":36212,"ancestor_record_offset":34444,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":34444,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35488,"parent_record_offset":35420,"ancestor_record_offset":34444,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35572,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35896,"parent_record_offset":35872,"ancestor_record_offset":34444,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":164,"record_offset":35252,"parent_record_offset":35228,"ancestor_record_offset":34444,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":164,"record_offset":36116,"parent_record_offset":36092,"ancestor_record_offset":34444,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":164,"record_offset":36284,"parent_record_offset":36260,"ancestor_record_offset":34444,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":164,"record_offset":35872,"parent_record_offset":35848,"ancestor_record_offset":34444,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":164,"record_offset":35848,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":164,"record_offset":36092,"parent_record_offset":36068,"ancestor_record_offset":34444,"inlinee":13187,"source_line":60,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":164,"record_offset":36308,"parent_record_offset":36284,"ancestor_record_offset":34444,"inlinee":13172,"source_line":64,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":35448,"parent_record_offset":35420,"ancestor_record_offset":34444,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":164,"record_offset":36212,"parent_record_offset":36188,"ancestor_record_offset":34444,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":164,"record_offset":36260,"parent_record_offset":36236,"ancestor_record_offset":34444,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":35364,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":35932,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":36616,"parent_record_offset":36592,"ancestor_record_offset":34444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":164,"record_offset":34916,"parent_record_offset":34892,"ancestor_record_offset":34444,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":35016,"parent_record_offset":34956,"ancestor_record_offset":34444,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":34988,"parent_record_offset":34956,"ancestor_record_offset":34444,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":34868,"parent_record_offset":34844,"ancestor_record_offset":34444,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":164,"record_offset":34892,"parent_record_offset":34868,"ancestor_record_offset":34444,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":37224,"parent_record_offset":37160,"ancestor_record_offset":34444,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":34724,"parent_record_offset":34688,"ancestor_record_offset":34444,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":34444,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":34748,"parent_record_offset":34724,"ancestor_record_offset":34444,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::blur_downsample::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":35664,"parent_record_offset":35628,"ancestor_record_offset":34444,"inlinee":10195,"source_line":1192,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::blur_downsample::impl$7::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":35732,"parent_record_offset":35696,"ancestor_record_offset":34444,"inlinee":13987,"source_line":1238,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::screenshot::capture_screenshot_buffer::impl$7::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":36476,"parent_record_offset":36440,"ancestor_record_offset":34444,"inlinee":9764,"source_line":34094,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::screenshot::capture_screenshot_texture::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":36008,"parent_record_offset":35972,"ancestor_record_offset":34444,"inlinee":4554,"source_line":34162,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::screenshot::capture_screenshot_texture::impl$7::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":36720,"parent_record_offset":36684,"ancestor_record_offset":34444,"inlinee":13981,"source_line":34226,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":37132,"parent_record_offset":37104,"ancestor_record_offset":34444,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":34688,"parent_record_offset":34660,"ancestor_record_offset":34444,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":37160,"parent_record_offset":37132,"ancestor_record_offset":34444,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":34780,"parent_record_offset":34688,"ancestor_record_offset":34444,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":37196,"parent_record_offset":37160,"ancestor_record_offset":34444,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$66::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":164,"record_offset":35596,"parent_record_offset":35572,"ancestor_record_offset":34444,"inlinee":13942,"source_line":321,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35628,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":164,"record_offset":35528,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::export_with_access_type
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":164,"record_offset":36560,"parent_record_offset":36536,"ancestor_record_offset":34444,"inlinee":12390,"source_line":1341,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35792,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::impl$6::into_dispatch_count
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35816,"parent_record_offset":35792,"ancestor_record_offset":34444,"inlinee":14205,"source_line":86,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35764,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::div_ceil_dims
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35420,"parent_record_offset":35396,"ancestor_record_offset":34444,"inlinee":14911,"source_line":379,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::half_res
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35396,"parent_record_offset":35284,"ancestor_record_offset":34444,"inlinee":14910,"source_line":395,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture
// 记录形式: compiled_procedure; instances=1
// 源码行: 671
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":12432,"rva":4956464,"va":5373665584}
// 编译实例 RVAs: 0x4ba130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3674124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8f4efc7ff","size":5,"soft_fail":false,"target_va":5369992480,"va":5373666599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1836556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8f4f9e3ff","size":5,"soft_fail":false,"target_va":5371829632,"va":5373666183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42689891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e863658b02","size":5,"soft_fail":false,"target_va":5416356240,"va":5373666344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42955111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e867718f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5373666292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e836450000","size":5,"soft_fail":false,"target_va":5373684368,"va":5373666645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8be490000","size":5,"soft_fail":false,"target_va":5373684528,"va":5373665645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42954954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8ca708f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5373666417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42910516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e834c38e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373666311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42909808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e870c08e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373666331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10318800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8d0739d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373666043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10319159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e837759d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373666196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10297795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8c3219d00","size":5,"soft_fail":false,"target_va":5383963792,"va":5373665992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10306299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e8fb429d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5373665920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10298648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e818259d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5373666259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10297199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e86f1f9d00","size":5,"soft_fail":false,"target_va":5383962816,"va":5373665612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10087481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e839ec9900","size":5,"soft_fail":false,"target_va":5383753616,"va":5373666130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42952973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e80d698f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5373666382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::read_back_screenshot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373665584,"raw_hex":"e88eebffff","size":5,"soft_fail":false,"target_va":5373660912,"va":5373666141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":14952,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":15156,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":15216,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":15064,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":14796,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":13988,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":12596,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":12976,"parent_record_offset":12432,"ancestor_record_offset":12432,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14316,"parent_record_offset":14288,"ancestor_record_offset":12432,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14344,"parent_record_offset":14316,"ancestor_record_offset":12432,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14264,"parent_record_offset":14236,"ancestor_record_offset":12432,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14288,"parent_record_offset":14264,"ancestor_record_offset":12432,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14404,"parent_record_offset":14288,"ancestor_record_offset":12432,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":12432,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":12432,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":164,"record_offset":14136,"parent_record_offset":14096,"ancestor_record_offset":12432,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":12432,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":12432,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":14752,"parent_record_offset":14096,"ancestor_record_offset":12432,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":14468,"parent_record_offset":14236,"ancestor_record_offset":12432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":164,"record_offset":15024,"parent_record_offset":15000,"ancestor_record_offset":12432,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":15124,"parent_record_offset":15064,"ancestor_record_offset":12432,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":12432,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":13372,"parent_record_offset":13348,"ancestor_record_offset":12432,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":13580,"parent_record_offset":13552,"ancestor_record_offset":12432,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":13680,"parent_record_offset":13652,"ancestor_record_offset":12432,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":13652,"parent_record_offset":13624,"ancestor_record_offset":12432,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":12432,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":12432,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":12432,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":164,"record_offset":15000,"parent_record_offset":14976,"ancestor_record_offset":12432,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":12432,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":13300,"parent_record_offset":13184,"ancestor_record_offset":12432,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":12432,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":13012,"parent_record_offset":12976,"ancestor_record_offset":12432,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":13152,"parent_record_offset":13120,"ancestor_record_offset":12432,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":13120,"parent_record_offset":13084,"ancestor_record_offset":12432,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12432,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12432,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":13036,"parent_record_offset":13012,"ancestor_record_offset":12432,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":13624,"parent_record_offset":13084,"ancestor_record_offset":12432,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12700,"parent_record_offset":12668,"ancestor_record_offset":12432,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":12432,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":12432,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12876,"parent_record_offset":12824,"ancestor_record_offset":12432,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":12432,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12640,"parent_record_offset":12596,"ancestor_record_offset":12432,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":12900,"parent_record_offset":12876,"ancestor_record_offset":12432,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":13184,"parent_record_offset":13152,"ancestor_record_offset":12432,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":13468,"parent_record_offset":13184,"ancestor_record_offset":12432,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":13440,"parent_record_offset":13184,"ancestor_record_offset":12432,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":13244,"parent_record_offset":13184,"ancestor_record_offset":12432,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":15336,"parent_record_offset":15272,"ancestor_record_offset":12432,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":14860,"parent_record_offset":14824,"ancestor_record_offset":12432,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":12432,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":12432,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":15244,"parent_record_offset":15216,"ancestor_record_offset":12432,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":14824,"parent_record_offset":14796,"ancestor_record_offset":12432,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":15272,"parent_record_offset":15244,"ancestor_record_offset":12432,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":14916,"parent_record_offset":14824,"ancestor_record_offset":12432,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":15308,"parent_record_offset":15272,"ancestor_record_offset":12432,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":14024,"parent_record_offset":13988,"ancestor_record_offset":12432,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":15468,"parent_record_offset":15436,"ancestor_record_offset":12432,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":15908,"parent_record_offset":15704,"ancestor_record_offset":12432,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":164,"record_offset":14164,"parent_record_offset":14136,"ancestor_record_offset":12432,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":14236,"parent_record_offset":14096,"ancestor_record_offset":12432,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":14440,"parent_record_offset":14236,"ancestor_record_offset":12432,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":164,"record_offset":14192,"parent_record_offset":14164,"ancestor_record_offset":12432,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":14096,"parent_record_offset":14060,"ancestor_record_offset":12432,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":14060,"parent_record_offset":14024,"ancestor_record_offset":12432,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2A1D
// 首条位置: {"module":81,"inlinee":10781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 记录形式: compiled_procedure; instances=1
// 源码行: 706
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":46152,"rva":4974256,"va":5373683376}
// 编译实例 RVAs: 0x4be6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1994709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8d56f1e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373683990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140080620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4449375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8a11bbcff","size":5,"soft_fail":false,"target_va":5369234976,"va":5373684346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4378722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e89e2fbdff","size":5,"soft_fail":false,"target_va":5369305584,"va":5373684301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bd4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4199583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e861ebbfff","size":5,"soft_fail":false,"target_va":5369484448,"va":5373684026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d716a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42675443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8f32c8b02","size":5,"soft_fail":false,"target_va":5416359584,"va":5373684136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42892742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8c67d8e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373684085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42892035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8037b8e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373684104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10301324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e88c2f9d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373683519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10301612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8ac309d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373683743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::JoinInner$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-309326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373683376,"raw_hex":"e8b247fbff","size":5,"soft_fail":false,"target_va":5373374608,"va":5373683929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373683376,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373683963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":47976,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":164,"record_offset":48296,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":46896,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":164,"record_offset":46332,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":164,"record_offset":48268,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":47356,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":47240,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":47084,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":47544,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::thread::join_handle::JoinHandle::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\join_handle.rs
//     evidence={"module":164,"record_offset":48476,"parent_record_offset":46152,"ancestor_record_offset":46152,"inlinee":12195,"source_line":149,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":48716,"parent_record_offset":48692,"ancestor_record_offset":46152,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":48740,"parent_record_offset":48716,"ancestor_record_offset":46152,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":48764,"parent_record_offset":48740,"ancestor_record_offset":46152,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":48692,"parent_record_offset":48560,"ancestor_record_offset":46152,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":48560,"parent_record_offset":48532,"ancestor_record_offset":46152,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":46152,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":48140,"parent_record_offset":48116,"ancestor_record_offset":46152,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":48116,"parent_record_offset":48092,"ancestor_record_offset":46152,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":48068,"parent_record_offset":47976,"ancestor_record_offset":46152,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":46568,"parent_record_offset":46544,"ancestor_record_offset":46152,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":48012,"parent_record_offset":47976,"ancestor_record_offset":46152,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":48040,"parent_record_offset":47976,"ancestor_record_offset":46152,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":164,"record_offset":48412,"parent_record_offset":48320,"ancestor_record_offset":46152,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":164,"record_offset":49088,"parent_record_offset":49064,"ancestor_record_offset":46152,"inlinee":4794,"source_line":174,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":46544,"parent_record_offset":46444,"ancestor_record_offset":46152,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":46444,"parent_record_offset":46412,"ancestor_record_offset":46152,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":164,"record_offset":46928,"parent_record_offset":46896,"ancestor_record_offset":46152,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":164,"record_offset":46412,"parent_record_offset":46364,"ancestor_record_offset":46152,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":164,"record_offset":46364,"parent_record_offset":46332,"ancestor_record_offset":46152,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":48592,"parent_record_offset":48560,"ancestor_record_offset":46152,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":164,"record_offset":46960,"parent_record_offset":46928,"ancestor_record_offset":46152,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":48656,"parent_record_offset":48620,"ancestor_record_offset":46152,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":48620,"parent_record_offset":48592,"ancestor_record_offset":46152,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":46672,"parent_record_offset":46648,"ancestor_record_offset":46152,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":46488,"parent_record_offset":46444,"ancestor_record_offset":46152,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":46988,"parent_record_offset":46960,"ancestor_record_offset":46152,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":47048,"parent_record_offset":46928,"ancestor_record_offset":46152,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":46152,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":48532,"parent_record_offset":48504,"ancestor_record_offset":46152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":164,"record_offset":48228,"parent_record_offset":48204,"ancestor_record_offset":46152,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":164,"record_offset":49168,"parent_record_offset":49144,"ancestor_record_offset":46152,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":47320,"parent_record_offset":47240,"ancestor_record_offset":46152,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":49144,"parent_record_offset":49120,"ancestor_record_offset":46152,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":47288,"parent_record_offset":47240,"ancestor_record_offset":46152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":48436,"parent_record_offset":48412,"ancestor_record_offset":46152,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":46600,"parent_record_offset":46444,"ancestor_record_offset":46152,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":48180,"parent_record_offset":47976,"ancestor_record_offset":46152,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":46648,"parent_record_offset":46624,"ancestor_record_offset":46152,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":46624,"parent_record_offset":46600,"ancestor_record_offset":46152,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":164,"record_offset":48204,"parent_record_offset":48180,"ancestor_record_offset":46152,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":49120,"parent_record_offset":49088,"ancestor_record_offset":46152,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":48320,"parent_record_offset":48296,"ancestor_record_offset":46152,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47716,"parent_record_offset":47688,"ancestor_record_offset":46152,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47476,"parent_record_offset":47412,"ancestor_record_offset":46152,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47148,"parent_record_offset":47112,"ancestor_record_offset":46152,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47500,"parent_record_offset":47476,"ancestor_record_offset":46152,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47172,"parent_record_offset":47148,"ancestor_record_offset":46152,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":47744,"parent_record_offset":47716,"ancestor_record_offset":46152,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":47384,"parent_record_offset":47356,"ancestor_record_offset":46152,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":164,"record_offset":47688,"parent_record_offset":47660,"ancestor_record_offset":46152,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":47572,"parent_record_offset":47544,"ancestor_record_offset":46152,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":47660,"parent_record_offset":47628,"ancestor_record_offset":46152,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":47600,"parent_record_offset":47572,"ancestor_record_offset":46152,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":47628,"parent_record_offset":47600,"ancestor_record_offset":46152,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":47112,"parent_record_offset":47084,"ancestor_record_offset":46152,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":47412,"parent_record_offset":47384,"ancestor_record_offset":46152,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":47204,"parent_record_offset":47112,"ancestor_record_offset":46152,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":47448,"parent_record_offset":47412,"ancestor_record_offset":46152,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::read_back_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 733
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":536,"rva":4951792,"va":5373660912}
// 编译实例 RVAs: 0x4b8ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10986676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8b4a4a700","size":5,"soft_fail":false,"target_va":5384649296,"va":5373662615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10946073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e81906a700","size":5,"soft_fail":false,"target_va":5384609408,"va":5373663330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2016376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e878c41e00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373662307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2016863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e85fc61e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373661836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4940101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8bb9eb4ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373662288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4359823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e87179bdff","size":5,"soft_fail":false,"target_va":5369305584,"va":5373665402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4326476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8b4fbbdff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373665319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4302645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8cb58beff","size":5,"soft_fail":false,"target_va":5369360720,"va":5373663360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b65a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4205822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e802d3bfff","size":5,"soft_fail":false,"target_va":5369456032,"va":5373661849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6e30
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4206273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e83fd1bfff","size":5,"soft_fail":false,"target_va":5369458224,"va":5373664492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3671695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e871f9c7ff","size":5,"soft_fail":false,"target_va":5369992480,"va":5373664170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f98c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1834499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8fd01e4ff","size":5,"soft_fail":false,"target_va":5371828416,"va":5373662910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42692751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e88f708b02","size":5,"soft_fail":false,"target_va":5416356240,"va":5373663484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d716a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42695904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8e07c8b02","size":5,"soft_fail":false,"target_va":5416359584,"va":5373663675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43394722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8a2269602","size":5,"soft_fail":false,"target_va":5417057056,"va":5373662329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42702746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e89a978b02","size":5,"soft_fail":false,"target_va":5416365712,"va":5373662961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42958588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8fc7e8f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5373662201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10960732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e85c3fa700","size":5,"soft_fail":false,"target_va":5384624080,"va":5373663343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42957948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e87c7c8f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5373663455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42958127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e82f7d8f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5373663468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e81f4c0000","size":5,"soft_fail":false,"target_va":5373684432,"va":5373664940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42957679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e86f7b8f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5373663948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8ef5b0000","size":5,"soft_fail":false,"target_va":5373684528,"va":5373660988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t42957597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e81d7b8f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5373663774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42913121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e861cd8e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373663706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42912406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e896ca8e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373663733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10321953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e821809d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373662890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10322330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e89a819d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373663025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10302448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8f0339d00","size":5,"soft_fail":false,"target_va":5383963792,"va":5373661339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10310955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e82b559d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5373661264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10301795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e863319d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5373663112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10301856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8a0319d00","size":5,"soft_fail":false,"target_va":5383962816,"va":5373660955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-314414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e8d233fbff","size":5,"soft_fail":false,"target_va":5373348384,"va":5373662793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42955833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e839748f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5373663522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::write_image_with_callback_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373660912,"raw_hex":"e853430000","size":5,"soft_fail":false,"target_va":5373679168,"va":5373661928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%r8)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373660912,"raw_hex":"41ff5018","size":4,"soft_fail":false,"va":5373661804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":11112,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":2948,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":2908,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":11280,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":164,"record_offset":11168,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":164,"record_offset":11028,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":2100,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":704,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":1088,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":10356,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":14893,"source_line":104,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":3004,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":14892,"source_line":764,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7040,"parent_record_offset":6888,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":10524,"parent_record_offset":10496,"ancestor_record_offset":536,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$31::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":10496,"parent_record_offset":10356,"ancestor_record_offset":536,"inlinee":12245,"source_line":2239,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":536,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6888,"parent_record_offset":6852,"ancestor_record_offset":536,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":536,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6928,"parent_record_offset":6888,"ancestor_record_offset":536,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":7344,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":7376,"parent_record_offset":7344,"ancestor_record_offset":536,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":536,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":8120,"parent_record_offset":8084,"ancestor_record_offset":536,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":164,"record_offset":8084,"parent_record_offset":8048,"ancestor_record_offset":536,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":164,"record_offset":8048,"parent_record_offset":8012,"ancestor_record_offset":536,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":6476,"parent_record_offset":6416,"ancestor_record_offset":536,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9240,"parent_record_offset":9196,"ancestor_record_offset":536,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6736,"parent_record_offset":6700,"ancestor_record_offset":536,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9168,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":536,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":536,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":536,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":536,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":164,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":536,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":10556,"parent_record_offset":10524,"ancestor_record_offset":536,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":536,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":536,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":536,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":536,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":536,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2516,"parent_record_offset":2400,"ancestor_record_offset":536,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":536,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":164,"record_offset":8012,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":164,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":536,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":10620,"parent_record_offset":10584,"ancestor_record_offset":536,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":536,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":10584,"parent_record_offset":10556,"ancestor_record_offset":536,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":536,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":164,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":4608,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":10460,"parent_record_offset":10356,"ancestor_record_offset":536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":536,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":2864,"parent_record_offset":2208,"ancestor_record_offset":536,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":7168,"parent_record_offset":6764,"ancestor_record_offset":536,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":536,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":2580,"parent_record_offset":2348,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":536,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":536,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":1692,"parent_record_offset":1664,"ancestor_record_offset":536,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":9360,"parent_record_offset":9196,"ancestor_record_offset":536,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":8728,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":536,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":536,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":536,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":536,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":536,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":536,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":1412,"parent_record_offset":1296,"ancestor_record_offset":536,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":536,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":1124,"parent_record_offset":1088,"ancestor_record_offset":536,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":536,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":1232,"parent_record_offset":1196,"ancestor_record_offset":536,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":536,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":536,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":536,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":1736,"parent_record_offset":1196,"ancestor_record_offset":536,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":808,"parent_record_offset":776,"ancestor_record_offset":536,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":536,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":932,"parent_record_offset":900,"ancestor_record_offset":536,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":984,"parent_record_offset":932,"ancestor_record_offset":536,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":900,"parent_record_offset":872,"ancestor_record_offset":536,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":748,"parent_record_offset":704,"ancestor_record_offset":536,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":536,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":536,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":1580,"parent_record_offset":1296,"ancestor_record_offset":536,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":1552,"parent_record_offset":1296,"ancestor_record_offset":536,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":1356,"parent_record_offset":1296,"ancestor_record_offset":536,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":536,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9516,"parent_record_offset":9452,"ancestor_record_offset":536,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9076,"parent_record_offset":9040,"ancestor_record_offset":536,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":536,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":536,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":536,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":536,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":536,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":536,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":9012,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":536,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":164,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":536,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":164,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":536,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":164,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":536,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":164,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":536,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":536,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8884,"parent_record_offset":8852,"ancestor_record_offset":536,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":536,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":536,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8768,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":9040,"parent_record_offset":9012,"ancestor_record_offset":536,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":536,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":9132,"parent_record_offset":9040,"ancestor_record_offset":536,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":9488,"parent_record_offset":9452,"ancestor_record_offset":536,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":5580,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":2136,"parent_record_offset":2100,"ancestor_record_offset":536,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":3204,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":536,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":5216,"parent_record_offset":5000,"ancestor_record_offset":536,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":164,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":536,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":2348,"parent_record_offset":2208,"ancestor_record_offset":536,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":2552,"parent_record_offset":2348,"ancestor_record_offset":536,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":3592,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":536,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":6560,"parent_record_offset":6416,"ancestor_record_offset":536,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":164,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":536,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":164,"record_offset":2304,"parent_record_offset":2276,"ancestor_record_offset":536,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":7984,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":8636,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":536,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":2208,"parent_record_offset":2172,"ancestor_record_offset":536,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":536,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":6416,"parent_record_offset":3004,"ancestor_record_offset":536,"inlinee":14895,"source_line":67,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 764
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A2C
// 首条位置: {"module":164,"inlinee":14892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":7344,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9168,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":164,"record_offset":8012,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":4608,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":4668,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":8728,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":9012,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8768,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":5580,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":3204,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":3592,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":7984,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":8636,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":6416,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":14895,"source_line":67,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":3004,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":3004,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":3004,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":3004,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":3004,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":3004,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":7040,"parent_record_offset":6888,"ancestor_record_offset":3004,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":3004,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":3004,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":3004,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":3004,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":3004,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":3004,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":3004,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6888,"parent_record_offset":6852,"ancestor_record_offset":3004,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":3004,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":3004,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":6928,"parent_record_offset":6888,"ancestor_record_offset":3004,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":8168,"parent_record_offset":8120,"ancestor_record_offset":3004,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":7376,"parent_record_offset":7344,"ancestor_record_offset":3004,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":3004,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":164,"record_offset":8120,"parent_record_offset":8084,"ancestor_record_offset":3004,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":164,"record_offset":8084,"parent_record_offset":8048,"ancestor_record_offset":3004,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":164,"record_offset":8048,"parent_record_offset":8012,"ancestor_record_offset":3004,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":6476,"parent_record_offset":6416,"ancestor_record_offset":3004,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9240,"parent_record_offset":9196,"ancestor_record_offset":3004,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6736,"parent_record_offset":6700,"ancestor_record_offset":3004,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":3004,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":3004,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":3004,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":3004,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":164,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":3004,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":3004,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":3004,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":3004,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5032,"parent_record_offset":5000,"ancestor_record_offset":3004,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5064,"parent_record_offset":5032,"ancestor_record_offset":3004,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5180,"parent_record_offset":5064,"ancestor_record_offset":3004,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":164,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":3004,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":164,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":3004,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3004,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":3004,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":164,"record_offset":4900,"parent_record_offset":4852,"ancestor_record_offset":3004,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":3540,"parent_record_offset":3512,"ancestor_record_offset":3004,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":5528,"parent_record_offset":4852,"ancestor_record_offset":3004,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":164,"record_offset":7168,"parent_record_offset":6764,"ancestor_record_offset":3004,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":3004,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":4704,"parent_record_offset":4668,"ancestor_record_offset":3004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3004,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3004,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":164,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":3004,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":9360,"parent_record_offset":9196,"ancestor_record_offset":3004,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":3004,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":3004,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":164,"record_offset":4272,"parent_record_offset":4244,"ancestor_record_offset":3004,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3004,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3004,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":3004,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":164,"record_offset":3920,"parent_record_offset":3804,"ancestor_record_offset":3004,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":3004,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":164,"record_offset":3628,"parent_record_offset":3592,"ancestor_record_offset":3004,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":3772,"parent_record_offset":3740,"ancestor_record_offset":3004,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":164,"record_offset":3740,"parent_record_offset":3700,"ancestor_record_offset":3004,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":3004,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3004,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":3004,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":164,"record_offset":4244,"parent_record_offset":3700,"ancestor_record_offset":3004,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3308,"parent_record_offset":3276,"ancestor_record_offset":3004,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":3004,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3432,"parent_record_offset":3400,"ancestor_record_offset":3004,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3488,"parent_record_offset":3432,"ancestor_record_offset":3004,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":3004,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3248,"parent_record_offset":3204,"ancestor_record_offset":3004,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":164,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":3004,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":3804,"parent_record_offset":3772,"ancestor_record_offset":3004,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":4088,"parent_record_offset":3804,"ancestor_record_offset":3004,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":4060,"parent_record_offset":3804,"ancestor_record_offset":3004,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":164,"record_offset":3864,"parent_record_offset":3804,"ancestor_record_offset":3004,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":3004,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9516,"parent_record_offset":9452,"ancestor_record_offset":3004,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9076,"parent_record_offset":9040,"ancestor_record_offset":3004,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":3004,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":3004,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":3004,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":3004,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":3004,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":3004,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":164,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":3004,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":3004,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8884,"parent_record_offset":8852,"ancestor_record_offset":3004,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":3004,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":164,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":3004,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":9040,"parent_record_offset":9012,"ancestor_record_offset":3004,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":3004,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":9132,"parent_record_offset":9040,"ancestor_record_offset":3004,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":9488,"parent_record_offset":9452,"ancestor_record_offset":3004,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":4772,"parent_record_offset":4736,"ancestor_record_offset":3004,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":3004,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":5216,"parent_record_offset":5000,"ancestor_record_offset":3004,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":164,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":3004,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":5000,"parent_record_offset":4852,"ancestor_record_offset":3004,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":164,"record_offset":6060,"parent_record_offset":5844,"ancestor_record_offset":3004,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":3004,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":164,"record_offset":6560,"parent_record_offset":6416,"ancestor_record_offset":3004,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":164,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":3004,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":164,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":3004,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":164,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":3004,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":4852,"parent_record_offset":4808,"ancestor_record_offset":3004,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":164,"record_offset":4808,"parent_record_offset":4772,"ancestor_record_offset":3004,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 779
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42DB
// 首条位置: {"module":230,"inlinee":17115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":230,"record_offset":4340,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":4508,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":4396,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":4256,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4208,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4208,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":4208,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":230,"record_offset":4976,"parent_record_offset":4920,"ancestor_record_offset":4208,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":4208,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":4208,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":230,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":4208,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":4624,"parent_record_offset":4588,"ancestor_record_offset":4208,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":4588,"parent_record_offset":4560,"ancestor_record_offset":4208,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":4208,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":4208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":230,"record_offset":4656,"parent_record_offset":4624,"ancestor_record_offset":4208,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":4208,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":4208,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":4684,"parent_record_offset":4624,"ancestor_record_offset":4208,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":4716,"parent_record_offset":4588,"ancestor_record_offset":4208,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":4208,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":4208,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":4208,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_with_callback_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 803
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":164,"record_offset":37900,"rva":4970048,"va":5373679168}
// 编译实例 RVAs: 0x4bd640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10978549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8f584a700","size":5,"soft_fail":false,"target_va":5384659264,"va":5373680710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11009315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e823fda700","size":5,"soft_fail":false,"target_va":5384689616,"va":5373680296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10976099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8637ba700","size":5,"soft_fail":false,"target_va":5384656912,"va":5373680808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10976249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8f97ba700","size":5,"soft_fail":false,"target_va":5384656512,"va":5373680258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$image..codecs..jpeg..encoder..JpegEncoder$LT$W$GT$$u20$as$u20$image..image..ImageEncoder$GT$::write_image
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2626122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8b6edd7ff","size":5,"soft_fail":false,"target_va":5371053744,"va":5373679861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8310502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8e6ce7e00","size":5,"soft_fail":false,"target_va":5381991760,"va":5373681253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1999264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8a0811e00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373679419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t1999151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e82f811e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373679548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4957217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8df5bb4ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373679404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140082880
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4437932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e85448bcff","size":5,"soft_fail":false,"target_va":5369243776,"va":5373681703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140084fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=40 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4429408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8a069bcff","size":5,"soft_fail":false,"target_va":5369253824,"va":5373683227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4320440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e84813beff","size":5,"soft_fail":false,"target_va":5369361840,"va":5373682275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b65a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4223537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8cf8dbfff","size":5,"soft_fail":false,"target_va":5369456032,"va":5373679564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401463f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3635572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e88c86c8ff","size":5,"soft_fail":false,"target_va":5370045424,"va":5373680991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ebd00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2956118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8aae4d2ff","size":5,"soft_fail":false,"target_va":5370723584,"va":5373679697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405aaac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t969716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8f4cb0e00","size":5,"soft_fail":false,"target_va":5374651072,"va":5373681351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405aac80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t970005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e815cd0e00","size":5,"soft_fail":false,"target_va":5374651520,"va":5373681510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fbbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1302649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e879e01300","size":5,"soft_fail":false,"target_va":5374983152,"va":5373680498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406805e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1845304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e838281c00","size":5,"soft_fail":false,"target_va":5375526368,"va":5373681059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2148110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e80ec72000","size":5,"soft_fail":false,"target_va":5375829056,"va":5373680941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42633562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e85a898a02","size":5,"soft_fail":false,"target_va":5416312752,"va":5373679185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42637213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e89d978a02","size":5,"soft_fail":false,"target_va":5416316480,"va":5373679262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d672b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42636869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e845968a02","size":5,"soft_fail":false,"target_va":5416317616,"va":5373680742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68340
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42641534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e87ea88a02","size":5,"soft_fail":false,"target_va":5416321856,"va":5373680317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68550
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42641764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e864a98a02","size":5,"soft_fail":false,"target_va":5416322384,"va":5373680615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d685e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42640917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e815a68a02","size":5,"soft_fail":false,"target_va":5416322528,"va":5373681606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43377266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e872e29502","size":5,"soft_fail":false,"target_va":5417057056,"va":5373679785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42938948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e844328f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5373681816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11115482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8da9ba900","size":5,"soft_fail":false,"target_va":5384794864,"va":5373679377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10941118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8bef2a600","size":5,"soft_fail":false,"target_va":5384624080,"va":5373682957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42939761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e871358f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5373681834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42940335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8af378f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5373681868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42940362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8ca378f02","size":5,"soft_fail":false,"target_va":5416622320,"va":5373681953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::codecs::jpeg::encoder::JpegEncoder$LT$W$GT$::new_with_quality
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2630548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e86cdcd7ff","size":5,"soft_fail":false,"target_va":5371049216,"va":5373679759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::image::ImageFormat::from_path::inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9296863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8dfdb8d00","size":5,"soft_fail":false,"target_va":5382977744,"va":5373680876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42894942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e85e868e02","size":5,"soft_fail":false,"target_va":5416576832,"va":5373681885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42894231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e897838e02","size":5,"soft_fail":false,"target_va":5416576144,"va":5373681908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10304889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8793d9d00","size":5,"soft_fail":false,"target_va":5383984848,"va":5373679954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10305330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373679168,"raw_hex":"e8323f9d00","size":5,"soft_fail":false,"target_va":5383985360,"va":5373680025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*24(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373679168,"raw_hex":"ff5318","size":3,"soft_fail":false,"va":5373679517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":164,"record_offset":44040,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":164,"record_offset":44152,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":164,"record_offset":44124,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":46092,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":44744,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":14893,"source_line":104,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":38060,"parent_record_offset":37900,"ancestor_record_offset":37900,"inlinee":14914,"source_line":829,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":44324,"parent_record_offset":44300,"ancestor_record_offset":37900,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":44348,"parent_record_offset":44324,"ancestor_record_offset":37900,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39044,"parent_record_offset":39020,"ancestor_record_offset":37900,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39068,"parent_record_offset":39044,"ancestor_record_offset":37900,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":44372,"parent_record_offset":44348,"ancestor_record_offset":37900,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39092,"parent_record_offset":39068,"ancestor_record_offset":37900,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":44300,"parent_record_offset":44268,"ancestor_record_offset":37900,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39020,"parent_record_offset":38964,"ancestor_record_offset":37900,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":44268,"parent_record_offset":44224,"ancestor_record_offset":37900,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$17::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":164,"record_offset":44188,"parent_record_offset":44152,"ancestor_record_offset":37900,"inlinee":6855,"source_line":577,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":44224,"parent_record_offset":44188,"ancestor_record_offset":37900,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":41604,"parent_record_offset":41576,"ancestor_record_offset":37900,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$31::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":44888,"parent_record_offset":44744,"ancestor_record_offset":37900,"inlinee":12245,"source_line":2239,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":164,"record_offset":44088,"parent_record_offset":44064,"ancestor_record_offset":37900,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38992,"parent_record_offset":38964,"ancestor_record_offset":37900,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":37900,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38940,"parent_record_offset":38916,"ancestor_record_offset":37900,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":38684,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":38592,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":164,"record_offset":39780,"parent_record_offset":39736,"ancestor_record_offset":37900,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":164,"record_offset":38556,"parent_record_offset":38536,"ancestor_record_offset":37900,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":164,"record_offset":39736,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":164,"record_offset":44948,"parent_record_offset":44916,"ancestor_record_offset":37900,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":40584,"parent_record_offset":40556,"ancestor_record_offset":37900,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":45012,"parent_record_offset":44976,"ancestor_record_offset":37900,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":37900,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":164,"record_offset":44976,"parent_record_offset":44948,"ancestor_record_offset":37900,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":38408,"parent_record_offset":38388,"ancestor_record_offset":37900,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":41176,"parent_record_offset":41144,"ancestor_record_offset":37900,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":38432,"parent_record_offset":38408,"ancestor_record_offset":37900,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":37900,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38476,"parent_record_offset":38316,"ancestor_record_offset":37900,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":44064,"parent_record_offset":44040,"ancestor_record_offset":37900,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38512,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":44788,"parent_record_offset":44744,"ancestor_record_offset":37900,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":44844,"parent_record_offset":44744,"ancestor_record_offset":37900,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":40556,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":38868,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":38648,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":39708,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":40632,"parent_record_offset":40608,"ancestor_record_offset":37900,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":40656,"parent_record_offset":40632,"ancestor_record_offset":37900,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41088,"parent_record_offset":40800,"ancestor_record_offset":37900,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41144,"parent_record_offset":41116,"ancestor_record_offset":37900,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41116,"parent_record_offset":41088,"ancestor_record_offset":37900,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":39272,"parent_record_offset":39208,"ancestor_record_offset":37900,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":38776,"parent_record_offset":38740,"ancestor_record_offset":37900,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":39296,"parent_record_offset":39272,"ancestor_record_offset":37900,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":38800,"parent_record_offset":38776,"ancestor_record_offset":37900,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38316,"parent_record_offset":38284,"ancestor_record_offset":37900,"inlinee":9682,"source_line":802,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38284,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":9736,"source_line":674,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38340,"parent_record_offset":38316,"ancestor_record_offset":37900,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40800,"parent_record_offset":40764,"ancestor_record_offset":37900,"inlinee":9631,"source_line":713,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::write_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40764,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":9737,"source_line":1049,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40884,"parent_record_offset":40860,"ancestor_record_offset":37900,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":37900,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38620,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::write_buffer_with_format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":164,"record_offset":41220,"parent_record_offset":40764,"ancestor_record_offset":37900,"inlinee":9685,"source_line":1156,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::image::ImageFormat::from_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":164,"record_offset":40700,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":14925,"source_line":126,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":38712,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":39180,"parent_record_offset":39152,"ancestor_record_offset":37900,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":38740,"parent_record_offset":38712,"ancestor_record_offset":37900,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":39208,"parent_record_offset":39180,"ancestor_record_offset":37900,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":38832,"parent_record_offset":38740,"ancestor_record_offset":37900,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":39244,"parent_record_offset":39208,"ancestor_record_offset":37900,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":40608,"parent_record_offset":40584,"ancestor_record_offset":37900,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39676,"parent_record_offset":39624,"ancestor_record_offset":37900,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39648,"parent_record_offset":39624,"ancestor_record_offset":37900,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39624,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::impl$6::flush
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":164,"record_offset":42280,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":14926,"source_line":643,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":164,"record_offset":41420,"parent_record_offset":41396,"ancestor_record_offset":37900,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39836,"parent_record_offset":39812,"ancestor_record_offset":37900,"inlinee":14919,"source_line":307,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::tempfile_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39892,"parent_record_offset":39812,"ancestor_record_offset":37900,"inlinee":13846,"source_line":555,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::file::NamedTempFile::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/file/mod.rs
//     evidence={"module":164,"record_offset":39812,"parent_record_offset":38060,"ancestor_record_offset":37900,"inlinee":14918,"source_line":574,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39860,"parent_record_offset":39836,"ancestor_record_offset":37900,"inlinee":14921,"source_line":228,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":38536,"parent_record_offset":38512,"ancestor_record_offset":37900,"inlinee":14916,"source_line":839,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 829
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A42
// 首条位置: {"module":164,"inlinee":14914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":164,"record_offset":38684,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":164,"record_offset":38592,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":164,"record_offset":39736,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38512,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":40556,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":38868,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":38648,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":164,"record_offset":39708,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38284,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":9736,"source_line":674,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::write_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40764,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":9737,"source_line":1049,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38620,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":9744,"source_line":1110,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] image::image::ImageFormat::from_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":164,"record_offset":40700,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":14925,"source_line":126,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":38712,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39624,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufwriter::impl$6::flush
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":164,"record_offset":42280,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":14926,"source_line":643,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] tempfile::file::NamedTempFile::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/file/mod.rs
//     evidence={"module":164,"record_offset":39812,"parent_record_offset":38060,"ancestor_record_offset":38060,"inlinee":14918,"source_line":574,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39044,"parent_record_offset":39020,"ancestor_record_offset":38060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39068,"parent_record_offset":39044,"ancestor_record_offset":38060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39092,"parent_record_offset":39068,"ancestor_record_offset":38060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":164,"record_offset":39020,"parent_record_offset":38964,"ancestor_record_offset":38060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":164,"record_offset":41604,"parent_record_offset":41576,"ancestor_record_offset":38060,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38992,"parent_record_offset":38964,"ancestor_record_offset":38060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":38060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":164,"record_offset":38940,"parent_record_offset":38916,"ancestor_record_offset":38060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":164,"record_offset":39780,"parent_record_offset":39736,"ancestor_record_offset":38060,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":164,"record_offset":38556,"parent_record_offset":38536,"ancestor_record_offset":38060,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":40584,"parent_record_offset":40556,"ancestor_record_offset":38060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":38408,"parent_record_offset":38388,"ancestor_record_offset":38060,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":41176,"parent_record_offset":41144,"ancestor_record_offset":38060,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":38432,"parent_record_offset":38408,"ancestor_record_offset":38060,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":38060,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":38476,"parent_record_offset":38316,"ancestor_record_offset":38060,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":164,"record_offset":38892,"parent_record_offset":38868,"ancestor_record_offset":38060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":40632,"parent_record_offset":40608,"ancestor_record_offset":38060,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":164,"record_offset":40656,"parent_record_offset":40632,"ancestor_record_offset":38060,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41088,"parent_record_offset":40800,"ancestor_record_offset":38060,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41144,"parent_record_offset":41116,"ancestor_record_offset":38060,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":164,"record_offset":41116,"parent_record_offset":41088,"ancestor_record_offset":38060,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":39272,"parent_record_offset":39208,"ancestor_record_offset":38060,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":38776,"parent_record_offset":38740,"ancestor_record_offset":38060,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":39296,"parent_record_offset":39272,"ancestor_record_offset":38060,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":164,"record_offset":38800,"parent_record_offset":38776,"ancestor_record_offset":38060,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38316,"parent_record_offset":38284,"ancestor_record_offset":38060,"inlinee":9682,"source_line":802,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38340,"parent_record_offset":38316,"ancestor_record_offset":38060,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40800,"parent_record_offset":40764,"ancestor_record_offset":38060,"inlinee":9631,"source_line":713,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":40884,"parent_record_offset":40860,"ancestor_record_offset":38060,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":164,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":38060,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::write_buffer_with_format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":164,"record_offset":41220,"parent_record_offset":40764,"ancestor_record_offset":38060,"inlinee":9685,"source_line":1156,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":164,"record_offset":39180,"parent_record_offset":39152,"ancestor_record_offset":38060,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":38740,"parent_record_offset":38712,"ancestor_record_offset":38060,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":164,"record_offset":39208,"parent_record_offset":39180,"ancestor_record_offset":38060,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":38832,"parent_record_offset":38740,"ancestor_record_offset":38060,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":164,"record_offset":39244,"parent_record_offset":39208,"ancestor_record_offset":38060,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":164,"record_offset":40608,"parent_record_offset":40584,"ancestor_record_offset":38060,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39676,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":164,"record_offset":39648,"parent_record_offset":39624,"ancestor_record_offset":38060,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":164,"record_offset":41420,"parent_record_offset":41396,"ancestor_record_offset":38060,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39836,"parent_record_offset":39812,"ancestor_record_offset":38060,"inlinee":14919,"source_line":307,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::tempfile_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39892,"parent_record_offset":39812,"ancestor_record_offset":38060,"inlinee":13846,"source_line":555,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":164,"record_offset":39860,"parent_record_offset":39836,"ancestor_record_offset":38060,"inlinee":14921,"source_line":228,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":164,"record_offset":38536,"parent_record_offset":38512,"ancestor_record_offset":38060,"inlinee":14916,"source_line":839,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 839
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A44
// 首条位置: {"module":164,"inlinee":14916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":164,"record_offset":38556,"parent_record_offset":38536,"ancestor_record_offset":38536,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::progressive_blur_downsample
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 892
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A3D
// 首条位置: {"module":164,"inlinee":14909}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35572,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":164,"record_offset":35848,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":35364,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":164,"record_offset":35932,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35628,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":164,"record_offset":35528,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35792,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35764,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::half_res
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35396,"parent_record_offset":35284,"ancestor_record_offset":35284,"inlinee":14910,"source_line":395,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35488,"parent_record_offset":35420,"ancestor_record_offset":35284,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":164,"record_offset":35896,"parent_record_offset":35872,"ancestor_record_offset":35284,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":164,"record_offset":35872,"parent_record_offset":35848,"ancestor_record_offset":35284,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":164,"record_offset":35448,"parent_record_offset":35420,"ancestor_record_offset":35284,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::blur_downsample::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":35664,"parent_record_offset":35628,"ancestor_record_offset":35284,"inlinee":10195,"source_line":1192,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::blur_downsample::impl$7::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":164,"record_offset":35732,"parent_record_offset":35696,"ancestor_record_offset":35284,"inlinee":13987,"source_line":1238,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$66::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":164,"record_offset":35596,"parent_record_offset":35572,"ancestor_record_offset":35284,"inlinee":13942,"source_line":321,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::impl$6::into_dispatch_count
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":164,"record_offset":35816,"parent_record_offset":35792,"ancestor_record_offset":35284,"inlinee":14205,"source_line":86,"archive":"_pdb/C/modules/00164.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::div_ceil_dims
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":164,"record_offset":35420,"parent_record_offset":35396,"ancestor_record_offset":35284,"inlinee":14911,"source_line":379,"archive":"_pdb/C/modules/00164.jsonl.gz"}
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
// 完整结构及原始类型记录: screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ActiveScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ActiveScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "tiny_glade::screenshot::finish_screenshot_capture::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::BeautyShotState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::BeautyShotState>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::BeautyShotState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<tiny_glade::screenshot::BeautyShotState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$132::init_access<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState> >", "constructor": "bevy_ecs::system::system_param::impl$132::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >", "bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotCapturedCallback
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotCapturedCallback"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::tgsr_capture::TgsrCapturePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::tgsr_capture::TgsrCapturePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotSink
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::ScreenshotSink>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::ScreenshotSink"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::screenshot::ScreenshotSink>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: screenshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 76; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 信息 ID: 8e46c6a256922b9d13bf985a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=164, inline_record=10356; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=164, inline_record=44744; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=164, inline_record=45420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 信息 ID: 8bf76b0f07d29eddecadefb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=164, inline_record=23356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 信息 ID: 9a5aca35247077a84cdf2684
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=164, inline_record=19076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 信息 ID: d0274046f214ea1889287a89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::add_captured_screenshots_to_steam_library", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0 <- alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0x2f92b6 -> 0x2f7c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0x2f9340 -> 0x2f7c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request
// 信息 ID: ce19662d34eb9182047159b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=164, inline_record=16648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 信息 ID: db78daa861dc8e4c7885fd68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::consume_steam_screenshot_request", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=164, inline_record=17072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture
// 信息 ID: c531b7af3e31accbe2eaa0eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56f82d -> 0x4ba130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 信息 ID: 43b33cb545dc4958d54423ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::finish_screenshot_capture", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::finish_screenshot_capture::closure_env$0 <- alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=81, inline_record=42192; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=81, inline_record=42328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests
// 信息 ID: 865cdebe7a708b182249b5f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e7ceb139f4de83eee8a8ce93: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > @ 0x2ae5cb -> 0x4ba5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> @ 0x3204d0 -> 0x4ba5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 信息 ID: 95a243e21999fa898459d018
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=164, inline_record=19036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 信息 ID: aabd96d73a551ce637e4f71d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u16,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u16,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; module=86, inline_record=59984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::handle_screenshot_opportunity
// 信息 ID: 4dcabbc1afb170cba7c40986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a33d -> 0x4bc510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a448 -> 0x4bc510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a5d4 -> 0x4bc510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a730 -> 0x4bc510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a84c -> 0x4bc510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 信息 ID: eb9eb16e20149a8679d6b25b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8822ccb8294ea63b3ab9ae31: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>)> > @ 0x2abd9a -> 0x4be6b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>)> @ 0x3206a2 -> 0x4be6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$0::build
// 信息 ID: 6c80720e56b06711ae5f1dc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$11::clone
// 信息 ID: 9b1823cb236e0823f9101bba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=164, inline_record=6416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$1::drop
// 信息 ID: ab8e7611efee48a848b47e82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>; module=6, inline_record=1320816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$6::eq
// 信息 ID: f7644cc50953227345661de1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=164, inline_record=35048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::next_requested_screenshot_stage
// 信息 ID: 92e1ecb00e869d8ed19092ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=107, inline_record=61592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::progressive_blur_downsample
// 信息 ID: 7ca0aa1789296b0002881838
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=164, inline_record=35284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::read_back_screenshot
// 信息 ID: d323e6a07b07e102b6bdd46c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x4ba35d -> 0x4b8ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x4ba371 -> 0x4b8ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image
// 信息 ID: b1b8247680a326c6593e519b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=164, inline_record=3004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image::closure$0
// 信息 ID: bd4001e5b2a4198c7cacf036
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::MaybeDangling<tiny_glade::screenshot::write_image::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::screenshot::write_image::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; module=230, inline_record=4208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl
// 信息 ID: 4ef091e3097c2a9507ed0275
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=164, inline_record=38060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl::closure$0
// 信息 ID: df0bca345bf931df798bfa32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=164, inline_record=38536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_with_callback_impl
// 信息 ID: 0b57d2de43e047646a35d8fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::read_back_screenshot @ 0x4b92e8 -> 0x4bd640 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0x5e694a -> 0x4bd640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
