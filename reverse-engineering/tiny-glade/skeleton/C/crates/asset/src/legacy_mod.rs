// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/legacy_mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 102df410907a0fb8d77cde59a205930143a59c94e72173e03f68d0db3a42b252
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: legacy_mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1771
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::legacy_mod::is_shipped_era_mod_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1771,"record_offset":764,"rva":28201312,"va":5396910432}
// 编译实例 RVAs: 0x1ae5160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::legacy_mod::flip_triangle_winding
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1771,"record_offset":260,"rva":28201104,"va":5396910224}
// 编译实例 RVAs: 0x1ae5090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1771,"record_offset":392,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9628,"source_line":1290,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1771,"record_offset":584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6601,"source_line":905,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1771,"record_offset":440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1771,"record_offset":708,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1771,"record_offset":664,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1771,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1771,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":6602,"source_line":1299,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1771,"record_offset":476,"parent_record_offset":440,"ancestor_record_offset":260,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1771,"record_offset":516,"parent_record_offset":476,"ancestor_record_offset":260,"inlinee":11985,"source_line":2092,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExactMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1771,"record_offset":412,"parent_record_offset":392,"ancestor_record_offset":260,"inlinee":9630,"source_line":2028,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::legacy_mod::needs_legacy_textured_upgrade
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1771,"record_offset":1912,"rva":28201552,"va":5396910672}
// 编译实例 RVAs: 0x1ae5250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1771,"record_offset":2044,"parent_record_offset":1912,"ancestor_record_offset":1912,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":2196,"parent_record_offset":2176,"ancestor_record_offset":1912,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1912,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1771,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1912,"inlinee":13096,"source_line":1880,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1771,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1912,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1912,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1912,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":2176,"parent_record_offset":2088,"ancestor_record_offset":1912,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1912,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1771,"record_offset":2088,"parent_record_offset":2068,"ancestor_record_offset":1912,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1771,"record_offset":2120,"parent_record_offset":2088,"ancestor_record_offset":1912,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::legacy_mod::legacy_textured_clutter_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1771,"record_offset":884,"rva":28201328,"va":5396910448}
// 编译实例 RVAs: 0x1ae5170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21231813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396910448,"raw_hex":"e83b07bcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396910496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: legacy_mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28188296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396910448,"raw_hex":"e878e151fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396910483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19710135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396910448,"raw_hex":"e8b7c02c01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396910654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1771,"record_offset":1016,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1771,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":884,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1771,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":884,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1771,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":884,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1771,"record_offset":1280,"parent_record_offset":1248,"ancestor_record_offset":884,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1771,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":884,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1771,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":884,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1771,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":884,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1771,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":884,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1771,"record_offset":1136,"parent_record_offset":1100,"ancestor_record_offset":884,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1771,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":884,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1771,"record_offset":1044,"parent_record_offset":1016,"ancestor_record_offset":884,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1771,"record_offset":1776,"parent_record_offset":1756,"ancestor_record_offset":884,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1771,"record_offset":1832,"parent_record_offset":1776,"ancestor_record_offset":884,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1771,"record_offset":1412,"parent_record_offset":1100,"ancestor_record_offset":884,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1771,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":884,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1771,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":884,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] asset::legacy_mod::legacy_textured_clutter_flags::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/legacy_mod.rs
//     evidence={"module":1771,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":884,"inlinee":86011,"source_line":58,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":884,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":884,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1771,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":884,"inlinee":13096,"source_line":1880,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1771,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":884,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1771,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":884,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1771,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":884,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1771,"record_offset":1756,"parent_record_offset":1540,"ancestor_record_offset":884,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1771,"record_offset":1804,"parent_record_offset":1776,"ancestor_record_offset":884,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":884,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":884,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1771,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":884,"inlinee":35488,"source_line":259,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::legacy_mod::legacy_textured_clutter_flags::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FFB
// 首条位置: {"module":1771,"inlinee":86011}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1771,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":1564,"inlinee":16063,"source_line":143,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1771,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":1564,"inlinee":13096,"source_line":1880,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":1564,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01771.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1771,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1564,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/01771.jsonl.gz"}
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
// 完整结构及原始类型记录: legacy_mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: legacy_mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: legacy_mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::legacy_mod::flip_triangle_winding
// 信息 ID: 3a78f39c291731abfb33afe4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da2b6 -> 0x1ae5090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::legacy_mod::is_shipped_era_mod_mesh
// 信息 ID: 6f89e0196f467e0128dd17ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da29b -> 0x1ae5160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::legacy_mod::legacy_textured_clutter_flags
// 信息 ID: cc65eb4dbb48aa81fc37a707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::legacy_mod_mesh::upgrade_legacy_textured_clutter_mesh @ 0x1b26912 -> 0x1ae5170 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> @ 0x1b29512 -> 0x1ae5170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::legacy_mod::legacy_textured_clutter_flags::closure$0
// 信息 ID: 36282b05afd6354d5b70ed88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::legacy_mod::legacy_textured_clutter_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::legacy_mod::legacy_textured_clutter_flags; module=1771, inline_record=1564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::legacy_mod::needs_legacy_textured_upgrade
// 信息 ID: 452288ee9a1285beb58d5fea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::legacy_mod_mesh::upgrade_legacy_textured_clutter_mesh @ 0x1b268f9 -> 0x1ae5250 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> @ 0x1b294e8 -> 0x1ae5250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
