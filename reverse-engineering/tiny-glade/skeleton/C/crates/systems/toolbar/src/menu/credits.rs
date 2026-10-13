// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/credits.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ef40ad1777ffcac702180cf26e27301ca58006f450ea5ca2f3431e85f1f4bdb0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: credits.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1235, 2868, 2873, 4850
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B35F
// 首条位置: {"module":2868,"inlinee":111455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2868,"record_offset":964,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2868,"record_offset":904,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2868,"record_offset":876,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":836,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":836,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":836,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1288,"parent_record_offset":1260,"ancestor_record_offset":836,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":836,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":836,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":836,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1088,"parent_record_offset":1048,"ancestor_record_offset":836,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":836,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":836,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":836,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2868,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":836,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":1176,"parent_record_offset":1144,"ancestor_record_offset":836,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2868,"record_offset":1416,"parent_record_offset":1144,"ancestor_record_offset":836,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":836,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":936,"parent_record_offset":904,"ancestor_record_offset":836,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2868,"record_offset":996,"parent_record_offset":964,"ancestor_record_offset":836,"inlinee":111457,"source_line":23,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B361
// 首条位置: {"module":2868,"inlinee":111457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2868,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":996,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":996,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":996,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":1288,"parent_record_offset":1260,"ancestor_record_offset":996,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":996,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":996,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":996,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1088,"parent_record_offset":1048,"ancestor_record_offset":996,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":996,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":996,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":996,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":1176,"parent_record_offset":1144,"ancestor_record_offset":996,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2868,"record_offset":1416,"parent_record_offset":1144,"ancestor_record_offset":996,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":3640,"rva":36651664,"va":5405360784}
// 编译实例 RVAs: 0x22f4290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-29686116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e89c063bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405364815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=7
//     target_aliases=6 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31882685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8438219fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405361720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31878986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8b69019fe","size":5,"soft_fail":false,"target_va":5373484384,"va":5405363365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31878620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8249219fe","size":5,"soft_fail":false,"target_va":5373484784,"va":5405363399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24297188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e81c418dfe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405361359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141176de0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18339731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e86d28e8fe","size":5,"soft_fail":false,"target_va":5387021792,"va":5405361518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18340836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e81c24e8fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5405364383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18338869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8cb2be8fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5405364320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18334530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8be3ce8fe","size":5,"soft_fail":false,"target_va":5387029552,"va":5405364077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411799d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18329008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e85052e8fe","size":5,"soft_fail":false,"target_va":5387033040,"va":5405362043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117aff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18322924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8146ae8fe","size":5,"soft_fail":false,"target_va":5387038704,"va":5405361623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117d940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18312223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8e193e8fe","size":5,"soft_fail":false,"target_va":5387049280,"va":5405361498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18159717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e89be7eafe","size":5,"soft_fail":false,"target_va":5387204688,"va":5405364400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18158633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8d7ebeafe","size":5,"soft_fail":false,"target_va":5387205712,"va":5405364340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10989959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e887b1a700","size":5,"soft_fail":false,"target_va":5416351392,"va":5405361428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11256541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8ddc2ab00","size":5,"soft_fail":false,"target_va":5416621408,"va":5405364862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t11258805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8b5cbab00","size":5,"soft_fail":false,"target_va":5416621600,"va":5405362790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e884fbffff","size":5,"soft_fail":false,"target_va":5405359728,"va":5405360871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11256397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e84dc2ab00","size":5,"soft_fail":false,"target_va":5416621376,"va":5405364974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21397440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e84080b9fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5405361227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21388933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e87ba1b9fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5405361152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21397432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e84880b9fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5405362339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21398035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8ed7db9fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5405360846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11291683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8234cac00","size":5,"soft_fail":false,"target_va":5416656560,"va":5405364872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18034058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e876d2ecfe","size":5,"soft_fail":false,"target_va":5387331072,"va":5405365125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11254441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8a9baab00","size":5,"soft_fail":false,"target_va":5416619360,"va":5405364914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23353115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8e5a89bfe","size":5,"soft_fail":false,"target_va":5382011344,"va":5405364454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29155031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8292143fe","size":5,"soft_fail":false,"target_va":5376206368,"va":5405361394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29155066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405360784,"raw_hex":"e8062143fe","size":5,"soft_fail":false,"target_va":5376208368,"va":5405363429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2868,"record_offset":6036,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":8672,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":5228,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":3828,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":4216,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen_without_block_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":6096,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":10271,"source_line":471,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":11188,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":10269,"source_line":505,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":9804,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":16916,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":12520,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":6588,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":7632,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15788,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15728,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":21734,"source_line":290,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15668,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15464,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15848,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::background_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2868,"record_offset":6412,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":111459,"source_line":304,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::max_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2868,"record_offset":6472,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":111461,"source_line":309,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2868,"record_offset":6532,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":111462,"source_line":314,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":3640,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":17172,"parent_record_offset":17148,"ancestor_record_offset":3640,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":17196,"parent_record_offset":17172,"ancestor_record_offset":3640,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":17124,"parent_record_offset":17068,"ancestor_record_offset":3640,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":6280,"parent_record_offset":6248,"ancestor_record_offset":3640,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":17096,"parent_record_offset":17068,"ancestor_record_offset":3640,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":3640,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":3640,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":3640,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":17044,"parent_record_offset":17020,"ancestor_record_offset":3640,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":6248,"parent_record_offset":6212,"ancestor_record_offset":3640,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":6188,"parent_record_offset":6120,"ancestor_record_offset":3640,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":3640,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2868,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":3640,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":3640,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":3640,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5504,"parent_record_offset":5476,"ancestor_record_offset":3640,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":3640,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5644,"parent_record_offset":5528,"ancestor_record_offset":3640,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2868,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":3640,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2868,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":3640,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2868,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":3640,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2868,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":3640,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2868,"record_offset":7428,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2868,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":3640,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2868,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":3640,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2868,"record_offset":7132,"parent_record_offset":7104,"ancestor_record_offset":3640,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2868,"record_offset":5376,"parent_record_offset":5336,"ancestor_record_offset":3640,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2868,"record_offset":7600,"parent_record_offset":6588,"ancestor_record_offset":3640,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2868,"record_offset":4164,"parent_record_offset":4136,"ancestor_record_offset":3640,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2868,"record_offset":5992,"parent_record_offset":5336,"ancestor_record_offset":3640,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":3640,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":5708,"parent_record_offset":5476,"ancestor_record_offset":3640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2868,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":3640,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2868,"record_offset":6628,"parent_record_offset":6588,"ancestor_record_offset":3640,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2868,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3640,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2868,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":3640,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2868,"record_offset":4820,"parent_record_offset":4792,"ancestor_record_offset":3640,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":6368,"parent_record_offset":6212,"ancestor_record_offset":3640,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2868,"record_offset":4920,"parent_record_offset":4892,"ancestor_record_offset":3640,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2868,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":3640,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2868,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3640,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2868,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3640,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2868,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":3640,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2868,"record_offset":4540,"parent_record_offset":4424,"ancestor_record_offset":3640,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2868,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":3640,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2868,"record_offset":4252,"parent_record_offset":4216,"ancestor_record_offset":3640,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2868,"record_offset":4392,"parent_record_offset":4360,"ancestor_record_offset":3640,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2868,"record_offset":4360,"parent_record_offset":4324,"ancestor_record_offset":3640,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2868,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":3640,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2868,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3640,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2868,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3640,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2868,"record_offset":4864,"parent_record_offset":4324,"ancestor_record_offset":3640,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":3932,"parent_record_offset":3900,"ancestor_record_offset":3640,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":3640,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":4056,"parent_record_offset":4024,"ancestor_record_offset":3640,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":4112,"parent_record_offset":4056,"ancestor_record_offset":3640,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":3640,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":3872,"parent_record_offset":3828,"ancestor_record_offset":3640,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2868,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3640,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2868,"record_offset":4424,"parent_record_offset":4392,"ancestor_record_offset":3640,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2868,"record_offset":4708,"parent_record_offset":4424,"ancestor_record_offset":3640,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2868,"record_offset":4680,"parent_record_offset":4424,"ancestor_record_offset":3640,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2868,"record_offset":4484,"parent_record_offset":4424,"ancestor_record_offset":3640,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2868,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":3640,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2868,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":3640,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2868,"record_offset":7104,"parent_record_offset":7072,"ancestor_record_offset":3640,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2868,"record_offset":7044,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2868,"record_offset":6904,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2868,"record_offset":7456,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2868,"record_offset":6988,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2868,"record_offset":6848,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2868,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":3640,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2868,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":3640,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2868,"record_offset":7312,"parent_record_offset":7204,"ancestor_record_offset":3640,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2868,"record_offset":6716,"parent_record_offset":6684,"ancestor_record_offset":3640,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":3640,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":7548,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":3640,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":3640,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":6744,"parent_record_offset":6684,"ancestor_record_offset":3640,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":3640,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":6876,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2868,"record_offset":7204,"parent_record_offset":6792,"ancestor_record_offset":3640,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":5264,"parent_record_offset":5228,"ancestor_record_offset":3640,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":8736,"parent_record_offset":8704,"ancestor_record_offset":3640,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":9188,"parent_record_offset":8984,"ancestor_record_offset":3640,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2868,"record_offset":5404,"parent_record_offset":5376,"ancestor_record_offset":3640,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":5476,"parent_record_offset":5336,"ancestor_record_offset":3640,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2868,"record_offset":5680,"parent_record_offset":5476,"ancestor_record_offset":3640,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2868,"record_offset":6656,"parent_record_offset":6588,"ancestor_record_offset":3640,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2868,"record_offset":5432,"parent_record_offset":5404,"ancestor_record_offset":3640,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2868,"record_offset":5336,"parent_record_offset":5300,"ancestor_record_offset":3640,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2868,"record_offset":5300,"parent_record_offset":5264,"ancestor_record_offset":3640,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":3640,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":15576,"parent_record_offset":15516,"ancestor_record_offset":3640,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":15516,"parent_record_offset":15464,"ancestor_record_offset":3640,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":15876,"parent_record_offset":15848,"ancestor_record_offset":3640,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2868,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":3640,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":6160,"parent_record_offset":6120,"ancestor_record_offset":3640,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":3640,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":12596,"parent_record_offset":12544,"ancestor_record_offset":3640,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":3640,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15696,"parent_record_offset":15668,"ancestor_record_offset":3640,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":15632,"parent_record_offset":15516,"ancestor_record_offset":3640,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":3640,"inlinee":111463,"source_line":229,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":6440,"parent_record_offset":6412,"ancestor_record_offset":3640,"inlinee":111460,"source_line":234,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111CB
// 首条位置: {"module":1235,"inlinee":70091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1235,"record_offset":142400,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1235,"record_offset":131200,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1235,"record_offset":141260,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1235,"record_offset":141232,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1235,"record_offset":133500,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":142988,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":134676,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":22162,"source_line":1481,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":135416,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":135204,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":15542,"source_line":2443,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":134984,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":16751,"source_line":2171,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":134732,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1235,"record_offset":130956,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":142344,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21727,"source_line":350,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":140120,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":135444,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":135556,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":138668,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":137120,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":137060,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21734,"source_line":290,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":137000,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":139940,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label_with_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":136824,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":10278,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$29::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":140176,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":70095,"source_line":216,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":137180,"parent_record_offset":130684,"ancestor_record_offset":130684,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":131492,"parent_record_offset":131468,"ancestor_record_offset":130684,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":131516,"parent_record_offset":131492,"ancestor_record_offset":130684,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":133388,"parent_record_offset":133364,"ancestor_record_offset":130684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":133412,"parent_record_offset":133388,"ancestor_record_offset":130684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":131540,"parent_record_offset":131516,"ancestor_record_offset":130684,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":133436,"parent_record_offset":133412,"ancestor_record_offset":130684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":131468,"parent_record_offset":131436,"ancestor_record_offset":130684,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":133364,"parent_record_offset":133308,"ancestor_record_offset":130684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131816,"parent_record_offset":131788,"ancestor_record_offset":130684,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131376,"parent_record_offset":131348,"ancestor_record_offset":130684,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131900,"parent_record_offset":131876,"ancestor_record_offset":130684,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":133336,"parent_record_offset":133308,"ancestor_record_offset":130684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":133308,"parent_record_offset":133284,"ancestor_record_offset":130684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131876,"parent_record_offset":131844,"ancestor_record_offset":130684,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131844,"parent_record_offset":131816,"ancestor_record_offset":130684,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131436,"parent_record_offset":131404,"ancestor_record_offset":130684,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":131404,"parent_record_offset":131376,"ancestor_record_offset":130684,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":133284,"parent_record_offset":133260,"ancestor_record_offset":130684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":142672,"parent_record_offset":142636,"ancestor_record_offset":130684,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1235,"record_offset":142456,"parent_record_offset":142428,"ancestor_record_offset":130684,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1235,"record_offset":142484,"parent_record_offset":142456,"ancestor_record_offset":130684,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1235,"record_offset":142512,"parent_record_offset":142484,"ancestor_record_offset":130684,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1235,"record_offset":142428,"parent_record_offset":142400,"ancestor_record_offset":130684,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1235,"record_offset":131660,"parent_record_offset":131200,"ancestor_record_offset":130684,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1235,"record_offset":131292,"parent_record_offset":131200,"ancestor_record_offset":130684,"inlinee":6749,"source_line":484,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131744,"parent_record_offset":131716,"ancestor_record_offset":130684,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131688,"parent_record_offset":131660,"ancestor_record_offset":130684,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131952,"parent_record_offset":131744,"ancestor_record_offset":130684,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131788,"parent_record_offset":131744,"ancestor_record_offset":130684,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131320,"parent_record_offset":131292,"ancestor_record_offset":130684,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1235,"record_offset":131348,"parent_record_offset":131320,"ancestor_record_offset":130684,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1235,"record_offset":131716,"parent_record_offset":131688,"ancestor_record_offset":130684,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1235,"record_offset":142696,"parent_record_offset":142672,"ancestor_record_offset":130684,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":133936,"parent_record_offset":133692,"ancestor_record_offset":130684,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":135896,"parent_record_offset":135872,"ancestor_record_offset":130684,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":135980,"parent_record_offset":135956,"ancestor_record_offset":130684,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":136396,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":142744,"parent_record_offset":142720,"ancestor_record_offset":130684,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1235,"record_offset":133572,"parent_record_offset":133536,"ancestor_record_offset":130684,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":142720,"parent_record_offset":142696,"ancestor_record_offset":130684,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":133792,"parent_record_offset":133764,"ancestor_record_offset":130684,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1235,"record_offset":134852,"parent_record_offset":134820,"ancestor_record_offset":130684,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":136128,"parent_record_offset":136100,"ancestor_record_offset":130684,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":136100,"parent_record_offset":136072,"ancestor_record_offset":130684,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1235,"record_offset":134084,"parent_record_offset":134060,"ancestor_record_offset":130684,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":132084,"parent_record_offset":132056,"ancestor_record_offset":130684,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":136568,"parent_record_offset":135556,"ancestor_record_offset":130684,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":134636,"parent_record_offset":133500,"ancestor_record_offset":130684,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1235,"record_offset":134936,"parent_record_offset":134908,"ancestor_record_offset":130684,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1235,"record_offset":142912,"parent_record_offset":142512,"ancestor_record_offset":130684,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1235,"record_offset":132320,"parent_record_offset":132296,"ancestor_record_offset":130684,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":135920,"parent_record_offset":135896,"ancestor_record_offset":130684,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":133212,"parent_record_offset":131200,"ancestor_record_offset":130684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":131980,"parent_record_offset":131744,"ancestor_record_offset":130684,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":132296,"parent_record_offset":132272,"ancestor_record_offset":130684,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":136244,"parent_record_offset":136220,"ancestor_record_offset":130684,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":135600,"parent_record_offset":135556,"ancestor_record_offset":130684,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":132272,"parent_record_offset":132248,"ancestor_record_offset":130684,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1235,"record_offset":133960,"parent_record_offset":133936,"ancestor_record_offset":130684,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1235,"record_offset":133984,"parent_record_offset":133960,"ancestor_record_offset":130684,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1235,"record_offset":134260,"parent_record_offset":134236,"ancestor_record_offset":130684,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1235,"record_offset":133740,"parent_record_offset":133692,"ancestor_record_offset":130684,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1235,"record_offset":135288,"parent_record_offset":135260,"ancestor_record_offset":130684,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":133824,"parent_record_offset":133764,"ancestor_record_offset":130684,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1235,"record_offset":133764,"parent_record_offset":133740,"ancestor_record_offset":130684,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1235,"record_offset":132224,"parent_record_offset":132188,"ancestor_record_offset":130684,"inlinee":7436,"source_line":137,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1235,"record_offset":132248,"parent_record_offset":132224,"ancestor_record_offset":130684,"inlinee":7437,"source_line":89,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1235,"record_offset":133860,"parent_record_offset":133692,"ancestor_record_offset":130684,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1235,"record_offset":133892,"parent_record_offset":133860,"ancestor_record_offset":130684,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":134820,"parent_record_offset":134792,"ancestor_record_offset":130684,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":131604,"parent_record_offset":131200,"ancestor_record_offset":130684,"inlinee":16753,"source_line":2107,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1235,"record_offset":134164,"parent_record_offset":134132,"ancestor_record_offset":130684,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$5::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1235,"record_offset":134132,"parent_record_offset":134108,"ancestor_record_offset":130684,"inlinee":12719,"source_line":85,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$6::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1235,"record_offset":134108,"parent_record_offset":134084,"ancestor_record_offset":130684,"inlinee":12717,"source_line":92,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":132056,"parent_record_offset":132032,"ancestor_record_offset":130684,"inlinee":16758,"source_line":1059,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::get_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":134024,"parent_record_offset":133644,"ancestor_record_offset":130684,"inlinee":12694,"source_line":642,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::next_inclusive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":133644,"parent_record_offset":133608,"ancestor_record_offset":130684,"inlinee":12795,"source_line":675,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$30::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":133536,"parent_record_offset":133500,"ancestor_record_offset":130684,"inlinee":12713,"source_line":1169,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$45::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":133608,"parent_record_offset":133572,"ancestor_record_offset":130684,"inlinee":12715,"source_line":1422,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1235,"record_offset":132032,"parent_record_offset":131200,"ancestor_record_offset":130684,"inlinee":16756,"source_line":511,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":133692,"parent_record_offset":133644,"ancestor_record_offset":130684,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":131628,"parent_record_offset":131604,"ancestor_record_offset":130684,"inlinee":9296,"source_line":975,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":135260,"parent_record_offset":135232,"ancestor_record_offset":130684,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":134236,"parent_record_offset":134212,"ancestor_record_offset":130684,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":135232,"parent_record_offset":135204,"ancestor_record_offset":130684,"inlinee":15545,"source_line":1017,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":134212,"parent_record_offset":134188,"ancestor_record_offset":130684,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1235,"record_offset":134188,"parent_record_offset":134164,"ancestor_record_offset":130684,"inlinee":12721,"source_line":611,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":135076,"parent_record_offset":135044,"ancestor_record_offset":130684,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":135044,"parent_record_offset":135012,"ancestor_record_offset":130684,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":134792,"parent_record_offset":134760,"ancestor_record_offset":130684,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":134884,"parent_record_offset":134792,"ancestor_record_offset":130684,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":134760,"parent_record_offset":134732,"ancestor_record_offset":130684,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1235,"record_offset":134908,"parent_record_offset":134884,"ancestor_record_offset":130684,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":131120,"parent_record_offset":131096,"ancestor_record_offset":130684,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":131144,"parent_record_offset":131120,"ancestor_record_offset":130684,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1235,"record_offset":136352,"parent_record_offset":136328,"ancestor_record_offset":130684,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":136040,"parent_record_offset":136012,"ancestor_record_offset":130684,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":136072,"parent_record_offset":136040,"ancestor_record_offset":130684,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":136012,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":135872,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":136424,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":135956,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1235,"record_offset":135816,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1235,"record_offset":136304,"parent_record_offset":136280,"ancestor_record_offset":130684,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":140308,"parent_record_offset":140272,"ancestor_record_offset":130684,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":135652,"parent_record_offset":135628,"ancestor_record_offset":130684,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":136280,"parent_record_offset":136172,"ancestor_record_offset":130684,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":135684,"parent_record_offset":135652,"ancestor_record_offset":130684,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":136220,"parent_record_offset":136196,"ancestor_record_offset":130684,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":136516,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":136196,"parent_record_offset":136172,"ancestor_record_offset":130684,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":135736,"parent_record_offset":135712,"ancestor_record_offset":130684,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":140364,"parent_record_offset":140308,"ancestor_record_offset":130684,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":135712,"parent_record_offset":135652,"ancestor_record_offset":130684,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":135760,"parent_record_offset":135736,"ancestor_record_offset":130684,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":135844,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":136172,"parent_record_offset":135760,"ancestor_record_offset":130684,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":1235,"record_offset":131096,"parent_record_offset":131072,"ancestor_record_offset":130684,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1235,"record_offset":130984,"parent_record_offset":130956,"ancestor_record_offset":130684,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1235,"record_offset":131072,"parent_record_offset":131040,"ancestor_record_offset":130684,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1235,"record_offset":131012,"parent_record_offset":130984,"ancestor_record_offset":130684,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1235,"record_offset":131040,"parent_record_offset":131012,"ancestor_record_offset":130684,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":140272,"parent_record_offset":140240,"ancestor_record_offset":130684,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":135628,"parent_record_offset":135556,"ancestor_record_offset":130684,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":140240,"parent_record_offset":140208,"ancestor_record_offset":130684,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":136908,"parent_record_offset":136848,"ancestor_record_offset":130684,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":136848,"parent_record_offset":136824,"ancestor_record_offset":130684,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":140208,"parent_record_offset":140176,"ancestor_record_offset":130684,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":137220,"parent_record_offset":137180,"ancestor_record_offset":130684,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":140144,"parent_record_offset":140120,"ancestor_record_offset":130684,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":142368,"parent_record_offset":142344,"ancestor_record_offset":130684,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":135492,"parent_record_offset":135468,"ancestor_record_offset":130684,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":135520,"parent_record_offset":135468,"ancestor_record_offset":130684,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":135468,"parent_record_offset":135444,"ancestor_record_offset":130684,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":137028,"parent_record_offset":137000,"ancestor_record_offset":130684,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label_with_id::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":136964,"parent_record_offset":136848,"ancestor_record_offset":130684,"inlinee":10238,"source_line":52,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111C0
// 首条位置: {"module":1235,"inlinee":70080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1235,"record_offset":120344,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1235,"record_offset":123108,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":120088,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":120284,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":119948,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":119664,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1235,"record_offset":120144,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1235,"record_offset":120004,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":119576,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":121592,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":21798,"source_line":357,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":120428,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":9899,"source_line":254,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":118492,"parent_record_offset":118360,"ancestor_record_offset":118360,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":119840,"parent_record_offset":119816,"ancestor_record_offset":118360,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":119864,"parent_record_offset":119840,"ancestor_record_offset":118360,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":119888,"parent_record_offset":119864,"ancestor_record_offset":118360,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":119816,"parent_record_offset":119760,"ancestor_record_offset":118360,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1235,"record_offset":120392,"parent_record_offset":120368,"ancestor_record_offset":118360,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":119788,"parent_record_offset":119760,"ancestor_record_offset":118360,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":119760,"parent_record_offset":119736,"ancestor_record_offset":118360,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1235,"record_offset":119736,"parent_record_offset":119712,"ancestor_record_offset":118360,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1235,"record_offset":123136,"parent_record_offset":123108,"ancestor_record_offset":118360,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":119324,"parent_record_offset":119300,"ancestor_record_offset":118360,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":118892,"parent_record_offset":118868,"ancestor_record_offset":118360,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":118976,"parent_record_offset":118952,"ancestor_record_offset":118360,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":119392,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":119124,"parent_record_offset":119096,"ancestor_record_offset":118360,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":119096,"parent_record_offset":119068,"ancestor_record_offset":118360,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":119972,"parent_record_offset":119948,"ancestor_record_offset":118360,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":120112,"parent_record_offset":120088,"ancestor_record_offset":118360,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":120368,"parent_record_offset":120344,"ancestor_record_offset":118360,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":121472,"parent_record_offset":120464,"ancestor_record_offset":118360,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":118916,"parent_record_offset":118892,"ancestor_record_offset":118360,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":119688,"parent_record_offset":119664,"ancestor_record_offset":118360,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":119240,"parent_record_offset":119216,"ancestor_record_offset":118360,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":120500,"parent_record_offset":120464,"ancestor_record_offset":118360,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":118548,"parent_record_offset":118520,"ancestor_record_offset":118360,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":120028,"parent_record_offset":120004,"ancestor_record_offset":118360,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":120052,"parent_record_offset":120028,"ancestor_record_offset":118360,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1235,"record_offset":119348,"parent_record_offset":119324,"ancestor_record_offset":118360,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":119036,"parent_record_offset":119008,"ancestor_record_offset":118360,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":119068,"parent_record_offset":119036,"ancestor_record_offset":118360,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":119008,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":118868,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":119420,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":118952,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1235,"record_offset":118812,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1235,"record_offset":119300,"parent_record_offset":119276,"ancestor_record_offset":118360,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":118648,"parent_record_offset":118608,"ancestor_record_offset":118360,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":120556,"parent_record_offset":120528,"ancestor_record_offset":118360,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":119276,"parent_record_offset":119168,"ancestor_record_offset":118360,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":118680,"parent_record_offset":118648,"ancestor_record_offset":118360,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":119216,"parent_record_offset":119192,"ancestor_record_offset":118360,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":119512,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":119192,"parent_record_offset":119168,"ancestor_record_offset":118360,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":118732,"parent_record_offset":118708,"ancestor_record_offset":118360,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":118708,"parent_record_offset":118648,"ancestor_record_offset":118360,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":120616,"parent_record_offset":120556,"ancestor_record_offset":118360,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":118756,"parent_record_offset":118732,"ancestor_record_offset":118360,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":118840,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":119168,"parent_record_offset":118756,"ancestor_record_offset":118360,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1235,"record_offset":120192,"parent_record_offset":120168,"ancestor_record_offset":118360,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1235,"record_offset":120168,"parent_record_offset":120144,"ancestor_record_offset":118360,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1235,"record_offset":120216,"parent_record_offset":120192,"ancestor_record_offset":118360,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":118608,"parent_record_offset":118576,"ancestor_record_offset":118360,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":120528,"parent_record_offset":120464,"ancestor_record_offset":118360,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":118576,"parent_record_offset":118520,"ancestor_record_offset":118360,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":121528,"parent_record_offset":121504,"ancestor_record_offset":118360,"inlinee":22043,"source_line":314,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":118520,"parent_record_offset":118492,"ancestor_record_offset":118360,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":120464,"parent_record_offset":120428,"ancestor_record_offset":118360,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":121552,"parent_record_offset":121528,"ancestor_record_offset":118360,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":119604,"parent_record_offset":119576,"ancestor_record_offset":118360,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":119628,"parent_record_offset":119604,"ancestor_record_offset":118360,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1235,"record_offset":121620,"parent_record_offset":121592,"ancestor_record_offset":118360,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ui::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":1235,"record_offset":121504,"parent_record_offset":120428,"ancestor_record_offset":118360,"inlinee":70082,"source_line":222,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111C2
// 首条位置: {"module":1235,"inlinee":70082}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":121528,"parent_record_offset":121504,"ancestor_record_offset":121504,"inlinee":22043,"source_line":314,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1235,"record_offset":121552,"parent_record_offset":121528,"ancestor_record_offset":121504,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":17940,"rva":36656512,"va":5405365632}
// 编译实例 RVAs: 0x22f5580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-29687180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e874023bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5405365863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36643661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e8b3dcd0fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5405365848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36027291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e86544dafd","size":5,"soft_fail":false,"target_va":5369338848,"va":5405366134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141180ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18303147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e855b7e8fe","size":5,"soft_fail":false,"target_va":5387062944,"va":5405366086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142957620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6692958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e85e206600","size":5,"soft_fail":false,"target_va":5412058656,"va":5405365693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e03090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11590083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e8c3d9b000","size":5,"soft_fail":false,"target_va":5416956048,"va":5405365960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11254748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e8dcbbab00","size":5,"soft_fail":false,"target_va":5416620794,"va":5405366041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11255604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e834bfab00","size":5,"soft_fail":false,"target_va":5416621632,"va":5405366023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6693449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405365632,"raw_hex":"e849226600","size":5,"soft_fail":false,"target_va":5412059264,"va":5405365810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2868,"record_offset":18308,"parent_record_offset":17940,"ancestor_record_offset":17940,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2868,"record_offset":18100,"parent_record_offset":17940,"ancestor_record_offset":17940,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2868,"record_offset":18776,"parent_record_offset":17940,"ancestor_record_offset":17940,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] std::thread::functions::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":2868,"record_offset":18136,"parent_record_offset":17940,"ancestor_record_offset":17940,"inlinee":7489,"source_line":125,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":17940,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":17940,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":17940,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":18572,"parent_record_offset":18544,"ancestor_record_offset":17940,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":17940,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":18544,"parent_record_offset":18512,"ancestor_record_offset":17940,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":18512,"parent_record_offset":18484,"ancestor_record_offset":17940,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":18364,"parent_record_offset":18336,"ancestor_record_offset":17940,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":18392,"parent_record_offset":18364,"ancestor_record_offset":17940,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":18420,"parent_record_offset":18392,"ancestor_record_offset":17940,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2868,"record_offset":18336,"parent_record_offset":18308,"ancestor_record_offset":17940,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":18456,"parent_record_offset":18420,"ancestor_record_offset":17940,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2868,"record_offset":18700,"parent_record_offset":18420,"ancestor_record_offset":17940,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":17940,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2868,"record_offset":18260,"parent_record_offset":18136,"ancestor_record_offset":17940,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2868,"record_offset":18220,"parent_record_offset":18192,"ancestor_record_offset":17940,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":2868,"record_offset":18168,"parent_record_offset":18136,"ancestor_record_offset":17940,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":2868,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":17940,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":18808,"rva":48246928,"va":5416956048}
// 编译实例 RVAs: 0x2e03090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-41277503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416956048,"raw_hex":"e8c1278afd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416956202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47707375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416956048,"raw_hex":"e8110b28fd","size":5,"soft_fail":false,"target_va":5369248944,"va":5416956314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-32161256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416956048,"raw_hex":"e8184215fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5416956115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-302034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416956048,"raw_hex":"e82e64fbff","size":5,"soft_fail":false,"target_va":5416654112,"va":5416956141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5416956048,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5416956178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2868,"record_offset":18972,"parent_record_offset":18808,"ancestor_record_offset":18808,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2868,"record_offset":19052,"parent_record_offset":18808,"ancestor_record_offset":18808,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":19080,"parent_record_offset":18808,"ancestor_record_offset":18808,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":19260,"parent_record_offset":19236,"ancestor_record_offset":18808,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":19284,"parent_record_offset":19260,"ancestor_record_offset":18808,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":18808,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":19236,"parent_record_offset":19104,"ancestor_record_offset":18808,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2868,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":18808,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2868,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":18808,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2868,"record_offset":19136,"parent_record_offset":19104,"ancestor_record_offset":18808,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2868,"record_offset":19200,"parent_record_offset":19164,"ancestor_record_offset":18808,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2868,"record_offset":19164,"parent_record_offset":19136,"ancestor_record_offset":18808,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2868,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":18808,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238CB
// 首条位置: {"module":4850,"inlinee":145611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4850,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/04850.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4850,"record_offset":1048,"rva":48345024,"va":5417054144}
// 编译实例 RVAs: 0x2e1afc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47715397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5417054144,"raw_hex":"e8bbeb27fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5417054240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-32259342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5417054144,"raw_hex":"e8f2c213fe","size":5,"soft_fail":false,"target_va":5384794864,"va":5417054201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-400120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5417054144,"raw_hex":"e808e5f9ff","size":5,"soft_fail":false,"target_va":5416654112,"va":5417054227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4850,"record_offset":1220,"parent_record_offset":1048,"ancestor_record_offset":1048,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/04850.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4850,"record_offset":1300,"parent_record_offset":1048,"ancestor_record_offset":1048,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/04850.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":4850,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":1048,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/04850.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4850,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":1048,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/04850.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$3::scroll<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1235,"record_offset":207200,"rva":18340160,"va":5387049280}
// 编译实例 RVAs: 0x117d940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11370962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e82e7e52ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387049645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18327443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e86d58e8fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387049630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13570358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e8caee30ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387049393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13564938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e8f60331ff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387049317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13564562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e86e0531ff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387049341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29570659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e86336c301","size":5,"soft_fail":false,"target_va":5416620769,"va":5387050105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5039358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e8021bb3ff","size":5,"soft_fail":false,"target_va":5382010272,"va":5387049625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29571944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e8683bc301","size":5,"soft_fail":false,"target_va":5416621536,"va":5387049587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29571529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e8c939c301","size":5,"soft_fail":false,"target_va":5416621600,"va":5387050066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10840994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387049280,"raw_hex":"e85e945aff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387049357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387049280,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5387049707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":207332,"parent_record_offset":207200,"ancestor_record_offset":207200,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":209256,"parent_record_offset":207200,"ancestor_record_offset":207200,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":208952,"parent_record_offset":208928,"ancestor_record_offset":207200,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":208976,"parent_record_offset":208952,"ancestor_record_offset":207200,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":209000,"parent_record_offset":208976,"ancestor_record_offset":207200,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":208928,"parent_record_offset":208896,"ancestor_record_offset":207200,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1235,"record_offset":208896,"parent_record_offset":208860,"ancestor_record_offset":207200,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1235,"record_offset":208860,"parent_record_offset":208520,"ancestor_record_offset":207200,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1235,"record_offset":208556,"parent_record_offset":208520,"ancestor_record_offset":207200,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1235,"record_offset":208724,"parent_record_offset":208700,"ancestor_record_offset":207200,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1235,"record_offset":209088,"parent_record_offset":207452,"ancestor_record_offset":207200,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1235,"record_offset":209112,"parent_record_offset":209088,"ancestor_record_offset":207200,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":1235,"record_offset":209140,"parent_record_offset":209112,"ancestor_record_offset":207200,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":209168,"parent_record_offset":209140,"ancestor_record_offset":207200,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1235,"record_offset":208240,"parent_record_offset":208216,"ancestor_record_offset":207200,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":207808,"parent_record_offset":207784,"ancestor_record_offset":207200,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1235,"record_offset":207892,"parent_record_offset":207868,"ancestor_record_offset":207200,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1235,"record_offset":208308,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":208040,"parent_record_offset":208012,"ancestor_record_offset":207200,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1235,"record_offset":208012,"parent_record_offset":207984,"ancestor_record_offset":207200,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":209060,"parent_record_offset":207452,"ancestor_record_offset":207200,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":208480,"parent_record_offset":207452,"ancestor_record_offset":207200,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1235,"record_offset":210264,"parent_record_offset":209256,"ancestor_record_offset":207200,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":207832,"parent_record_offset":207808,"ancestor_record_offset":207200,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":208676,"parent_record_offset":208652,"ancestor_record_offset":207200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1235,"record_offset":208156,"parent_record_offset":208132,"ancestor_record_offset":207200,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1235,"record_offset":207512,"parent_record_offset":207452,"ancestor_record_offset":207200,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208592,"parent_record_offset":208556,"ancestor_record_offset":207200,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208760,"parent_record_offset":208724,"ancestor_record_offset":207200,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208616,"parent_record_offset":208592,"ancestor_record_offset":207200,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208784,"parent_record_offset":208760,"ancestor_record_offset":207200,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208816,"parent_record_offset":208724,"ancestor_record_offset":207200,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1235,"record_offset":208264,"parent_record_offset":208240,"ancestor_record_offset":207200,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":207952,"parent_record_offset":207924,"ancestor_record_offset":207200,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":207984,"parent_record_offset":207952,"ancestor_record_offset":207200,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1235,"record_offset":207924,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":207784,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":208336,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1235,"record_offset":207868,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1235,"record_offset":207728,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1235,"record_offset":208216,"parent_record_offset":208192,"ancestor_record_offset":207200,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":207568,"parent_record_offset":207540,"ancestor_record_offset":207200,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":208192,"parent_record_offset":208084,"ancestor_record_offset":207200,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1235,"record_offset":207600,"parent_record_offset":207568,"ancestor_record_offset":207200,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":208132,"parent_record_offset":208108,"ancestor_record_offset":207200,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":208428,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":208108,"parent_record_offset":208084,"ancestor_record_offset":207200,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":207652,"parent_record_offset":207628,"ancestor_record_offset":207200,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":207628,"parent_record_offset":207568,"ancestor_record_offset":207200,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":207676,"parent_record_offset":207652,"ancestor_record_offset":207200,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":207756,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1235,"record_offset":208084,"parent_record_offset":207676,"ancestor_record_offset":207200,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1235,"record_offset":207540,"parent_record_offset":207452,"ancestor_record_offset":207200,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":207396,"parent_record_offset":207332,"ancestor_record_offset":207200,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::get_content_mut_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":207452,"parent_record_offset":207332,"ancestor_record_offset":207200,"inlinee":10210,"source_line":1579,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$18::get_content_mut_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":208520,"parent_record_offset":208480,"ancestor_record_offset":207200,"inlinee":10214,"source_line":1588,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":207372,"parent_record_offset":207332,"ancestor_record_offset":207200,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1235,"record_offset":207420,"parent_record_offset":207396,"ancestor_record_offset":207200,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::impl$0::create_node_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":1235,"record_offset":208652,"parent_record_offset":208520,"ancestor_record_offset":207200,"inlinee":70105,"source_line":298,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$7::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B369
// 首条位置: {"module":2868,"inlinee":111465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$0::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111D9
// 首条位置: {"module":1235,"inlinee":70105}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":208676,"parent_record_offset":208652,"ancestor_record_offset":208652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1235,"record_offset":208724,"parent_record_offset":208700,"ancestor_record_offset":208652,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":208700,"parent_record_offset":208676,"ancestor_record_offset":208652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208760,"parent_record_offset":208724,"ancestor_record_offset":208652,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208784,"parent_record_offset":208760,"ancestor_record_offset":208652,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1235,"record_offset":208816,"parent_record_offset":208724,"ancestor_record_offset":208652,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::background_color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B363
// 首条位置: {"module":2868,"inlinee":111459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":6440,"parent_record_offset":6412,"ancestor_record_offset":6412,"inlinee":111460,"source_line":234,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::max_scroll_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B365
// 首条位置: {"module":2868,"inlinee":111461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":6500,"parent_record_offset":6472,"ancestor_record_offset":6472,"inlinee":111460,"source_line":234,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B366
// 首条位置: {"module":2868,"inlinee":111462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2868,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6532,"inlinee":111463,"source_line":229,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":21904,"rva":36657760,"va":5405366880}
// 编译实例 RVAs: 0x22f5a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::prepare_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":19608,"rva":36657040,"va":5405366160}
// 编译实例 RVAs: 0x22f5790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22086943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366160,"raw_hex":"e8e1faaefe","size":5,"soft_fail":false,"target_va":5383279328,"va":5405366266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2868,"record_offset":19836,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":19752,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2868,"record_offset":19780,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$20::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":19864,"parent_record_offset":19608,"ancestor_record_offset":19608,"inlinee":21645,"source_line":1708,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2868,"record_offset":19896,"rva":36657296,"va":5405366416}
// 编译实例 RVAs: 0x22f5890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-29687874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e8beff3afe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405366573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5950
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35979518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e802ffdafd","size":5,"soft_fail":false,"target_va":5369387344,"va":5405366857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8f4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24536059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e8059c89fe","size":5,"soft_fail":false,"target_va":5380830416,"va":5405366470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23345374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e822c79bfe","size":5,"soft_fail":false,"target_va":5382021344,"va":5405366713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23341210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e866d79bfe","size":5,"soft_fail":false,"target_va":5382025312,"va":5405366517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23341185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e87fd79bfe","size":5,"soft_fail":false,"target_va":5382025280,"va":5405366460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23341740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405366416,"raw_hex":"e854d59bfe","size":5,"soft_fail":false,"target_va":5382024752,"va":5405366487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":20036,"parent_record_offset":19896,"ancestor_record_offset":19896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":20764,"parent_record_offset":19896,"ancestor_record_offset":19896,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":20940,"parent_record_offset":19896,"ancestor_record_offset":19896,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":20852,"parent_record_offset":19896,"ancestor_record_offset":19896,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2868,"record_offset":21028,"parent_record_offset":19896,"ancestor_record_offset":19896,"inlinee":21752,"source_line":1779,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":20368,"parent_record_offset":20344,"ancestor_record_offset":19896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":19896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":19896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2868,"record_offset":20344,"parent_record_offset":20288,"ancestor_record_offset":19896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":20316,"parent_record_offset":20288,"ancestor_record_offset":19896,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":20288,"parent_record_offset":20264,"ancestor_record_offset":19896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2868,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":19896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2868,"record_offset":20076,"parent_record_offset":20056,"ancestor_record_offset":19896,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2868,"record_offset":21056,"parent_record_offset":21028,"ancestor_record_offset":19896,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2868,"record_offset":20600,"parent_record_offset":20572,"ancestor_record_offset":19896,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2868,"record_offset":20968,"parent_record_offset":20940,"ancestor_record_offset":19896,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2868,"record_offset":20792,"parent_record_offset":20764,"ancestor_record_offset":19896,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2868,"record_offset":20880,"parent_record_offset":20852,"ancestor_record_offset":19896,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2868,"record_offset":20056,"parent_record_offset":20036,"ancestor_record_offset":19896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":21112,"parent_record_offset":21028,"ancestor_record_offset":19896,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02868.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2868,"record_offset":21080,"parent_record_offset":21056,"ancestor_record_offset":19896,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/02868.jsonl.gz"}
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
// 完整结构及原始类型记录: credits.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::CRATE_CREDITS
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["toolbar::menu::credits::CRATE_CREDITS::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::credits::ScrollContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content_mut<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content_mut", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any_mut<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any_mut", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::type_name<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::type_name", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::pre_update<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::pre_update", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::process_input_action<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::process_input_action", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ThirdPartyLicenseServer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>", "constructor": "core::option::Option", "arguments": ["toolbar::menu::credits::ThirdPartyLicenseServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >", "constructor": "enum2$", "arguments": ["core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::credits::ThirdPartyLicenseServer>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::credits::ThirdPartyLicenseServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$114::init_access<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32> >", "constructor": "bevy_ecs::system::system_param::impl$114::init_access", "arguments": ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: credits.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: credits.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<f32>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<f32>

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 信息 ID: 337ee1ac9c88c1c705c4c6f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::CRATE_CREDITS::closure_env$0 <- core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2868, inline_record=836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 信息 ID: d512e83dab4ac4dd745563e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2868, inline_record=996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::background_color
// 信息 ID: b275bc11b255bee623ad4a6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2868, inline_record=6412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::max_scroll_pos
// 信息 ID: 94ee657bbd2c76c606dba4f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2868, inline_record=6472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::offset
// 信息 ID: 36e34cdce2b079903b6f3f46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2868, inline_record=6532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$0::create_node_content
// 信息 ID: 0380145fdfbcfe64332b8554
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::impl$3::scroll<u64>; module=1235, inline_record=208652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::on_input
// 信息 ID: def77113b73f492bc6491a84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::prepare_layout
// 信息 ID: e8bf0b01437af6b32dd4b7a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::render
// 信息 ID: 77926eb0b53b5fc123df2cd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$3::scroll<u64>
// 信息 ID: f202881c62bd0580cb9e862d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::credits::impl$3::scroll -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::ui @ 0x22f455a -> 0x117d940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$7::default
// 信息 ID: 5fd71d65acf3ba258a0ea955
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server
// 信息 ID: 4ef51245a8d9affd25484468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1> @ 0x1178db5 -> 0x22f5580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 信息 ID: 80996df82477c6a9a5c3036e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::spawn_third_party_license_http_server @ 0x22f56c8 -> 0x2e03090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 信息 ID: 12289a2af45f2d709ebdfa28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::thread::lifecycle::spawn_unchecked<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::sys::backtrace::__rust_begin_short_backtrace<std::thread::lifecycle::spawn_unchecked::closure$1::closure$0::closure_env$0<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=4850, inline_record=952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 信息 ID: 4c1d4a52d0ce3b196983bb01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x2b2e530 -> 0x2e1afc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui
// 信息 ID: 8bddbfb2e78ca331993d3374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc8d029f3a5cd8d5737278ec: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> @ 0x11de4d7 -> 0x22f4290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > @ 0x230dcdb -> 0x22f4290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$0
// 信息 ID: 08d5a6e331ad4f56499eefc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1235, inline_record=130684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$1
// 信息 ID: af24cb2961229de15acc574f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$1 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1235, inline_record=118360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$1::closure$0
// 信息 ID: 0973b579ea1a08ea9f02715b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1235, inline_record=121504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
