// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/prefab.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e02c9c8e7fabe5a946ef2d08edba0683311c9fb5f1f9af4868b8e60dfa400495
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: prefab.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 504, 533, 534, 539, 544, 579, 592, 594, 614, 1643, 1656, 1658, 1660, 1682, 1683, 1684, 1685, 1805, 2630, 3378, 3380, 3900
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70A2
// 首条位置: {"module":504,"inlinee":28834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$55::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":504,"record_offset":46784,"parent_record_offset":46756,"ancestor_record_offset":46756,"inlinee":16017,"source_line":3555,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":504,"record_offset":46964,"parent_record_offset":46812,"ancestor_record_offset":46756,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$53::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":504,"record_offset":46812,"parent_record_offset":46784,"ancestor_record_offset":46756,"inlinee":16019,"source_line":3506,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":504,"record_offset":46868,"parent_record_offset":46840,"ancestor_record_offset":46756,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":504,"record_offset":46896,"parent_record_offset":46868,"ancestor_record_offset":46756,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":504,"record_offset":46924,"parent_record_offset":46896,"ancestor_record_offset":46756,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":504,"record_offset":46840,"parent_record_offset":46812,"ancestor_record_offset":46756,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$12::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14110
// 首条位置: {"module":1660,"inlinee":82192}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$67::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1660,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":13812,"inlinee":38905,"source_line":3841,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1660,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":13812,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1660,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":13812,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13812,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":13944,"parent_record_offset":13836,"ancestor_record_offset":13812,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13812,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13812,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1660,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13812,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":13812,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$13::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81EA
// 首条位置: {"module":539,"record_offset":129396,"rva":10939024,"va":5379648144}
// 编译实例 RVAs: 0xa6ea90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5172969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379648144,"raw_hex":"e8e9ee4e00","size":5,"soft_fail":false,"target_va":5384821184,"va":5379648210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=37
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7082
// 首条位置: {"module":504,"inlinee":28802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":504,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":13292,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":504,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":13292,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":504,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":13292,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$15::eq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E8D
// 首条位置: {"module":533,"inlinee":32397}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":533,"record_offset":152772,"parent_record_offset":152748,"ancestor_record_offset":152748,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":533,"record_offset":152796,"parent_record_offset":152772,"ancestor_record_offset":152748,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":533,"record_offset":152820,"parent_record_offset":152796,"ancestor_record_offset":152748,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":533,"record_offset":152848,"parent_record_offset":152820,"ancestor_record_offset":152748,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$18::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143E8
// 首条位置: {"module":1685,"inlinee":82920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::PrefabName::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143E1
// 首条位置: {"module":1684,"record_offset":736,"rva":25282928,"va":5393992048}
// 编译实例 RVAs: 0x181c970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18313412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393992048,"raw_hex":"e83c8fe8fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393992095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: prefab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25269897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393992048,"raw_hex":"e877697efe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393992084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: prefab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23064918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393992048,"raw_hex":"e856f15f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5393992133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22628596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393992048,"raw_hex":"e8f4485901","size":5,"soft_fail":false,"target_va":5416620769,"va":5393992168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9197337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393992048,"raw_hex":"e8e7a873ff","size":5,"soft_fail":false,"target_va":5384794736,"va":5393992068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1684,"record_offset":852,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1684,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":736,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1684,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":736,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1684,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":736,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1684,"record_offset":1060,"parent_record_offset":1040,"ancestor_record_offset":736,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":736,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":980,"parent_record_offset":948,"ancestor_record_offset":736,"inlinee":7216,"source_line":2245,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":736,"inlinee":7215,"source_line":2260,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":1236,"parent_record_offset":1004,"ancestor_record_offset":736,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":736,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":1040,"parent_record_offset":1004,"ancestor_record_offset":736,"inlinee":7219,"source_line":2249,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$31::from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":736,"inlinee":7214,"source_line":2352,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$70::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":736,"inlinee":7212,"source_line":3903,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$72::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":736,"inlinee":7210,"source_line":3941,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1684,"record_offset":1384,"parent_record_offset":948,"ancestor_record_offset":736,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1684,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":736,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1684,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":736,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1684,"record_offset":1180,"parent_record_offset":1004,"ancestor_record_offset":736,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::PrefabName::as_name_ref
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1684,"record_offset":536,"rva":25282912,"va":5393992032}
// 编译实例 RVAs: 0x181c960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::impl$5::as_ref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1684,"record_offset":660,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":82906,"source_line":70,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":700,"parent_record_offset":680,"ancestor_record_offset":536,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":680,"parent_record_offset":660,"ancestor_record_offset":536,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$5::as_ref
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 33, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143DA
// 首条位置: {"module":1684,"inlinee":82906}
// 编译实例 RVAs: 0x181c960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":376,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1684,"record_offset":396,"parent_record_offset":376,"ancestor_record_offset":260,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$4::as_ref
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1684,"record_offset":428,"rva":7119936,"va":5375829056}
// 编译实例 RVAs: 0x6ca440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::impl$6::borrow
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E8B
// 首条位置: {"module":533,"inlinee":32395}
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
// 完整结构及原始类型记录: prefab.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::PrefabName
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 48
// [PDB 泛型关系，非字段关系] {"type": "tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "constructor": "tuple$", "arguments": ["asset::prefab::PrefabName", "country_core::resources::prefab::Prefab"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::prefab::PrefabName", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["std::path::PathBuf", "alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::IntoIter<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::IntoIter<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<asset::prefab::PrefabName,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["asset::prefab::PrefabName", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<asset::prefab::PrefabName,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<asset::prefab::PrefabName,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::set::HashSet", "arguments": ["asset::prefab::PrefabName", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<asset::prefab::PrefabName>", "constructor": "ref$", "arguments": ["asset::prefab::PrefabName"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >", "constructor": "tuple$", "arguments": ["ref$<asset::prefab::PrefabName>", "ref$<asset::prefab::config::PrefabConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >", "constructor": "ref$", "arguments": ["tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "constructor": "tuple$", "arguments": ["ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name<ref$<asset::prefab::PrefabName> >", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> >", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add", "arguments": ["asset::prefab::PrefabName"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::Mutex", "arguments": ["std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["asset::prefab::PrefabName", "country_core::resources::prefab::Prefab", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<asset::prefab::PrefabName,tuple$<> >", "constructor": "tuple$", "arguments": ["asset::prefab::PrefabName", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["asset::prefab::PrefabName", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$13::clone_from<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::impl$13::clone_from", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "constructor": "tuple$", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::Iter<asset::prefab::PrefabName>", "constructor": "std::collections::hash::set::Iter", "arguments": ["asset::prefab::PrefabName"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["std::collections::hash::set::Iter<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >", "constructor": "hashbrown::set::impl$10::extend::closure_env$0", "arguments": ["asset::prefab::PrefabName", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >", "hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["asset::prefab::PrefabName", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["asset::prefab::PrefabName", "tuple$<asset::prefab::PrefabName,tuple$<> >", "tuple$<>", "hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["ref$<asset::prefab::PrefabName>", "asset::prefab::PrefabName", "tuple$<>", "asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>)", "core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$58::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > > >", "constructor": "hashbrown::map::impl$58::fold::closure_env$0", "arguments": ["asset::prefab::PrefabName", "tuple$<>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["asset::prefab::PrefabName", "tuple$<>", "tuple$<>", "hashbrown::map::impl$58::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$44::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "hashbrown::map::impl$44::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<asset::prefab::PrefabName,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<asset::prefab::PrefabName>,asset::prefab::PrefabName,tuple$<>,asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>),core::iter::adapters::map::map_fold::closure_env$0<asset::prefab::PrefabName,tuple$<asset::prefab::PrefabName,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<asset::prefab::PrefabName,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> >,hashbrown::set::impl$10::extend::closure_env$0<asset::prefab::PrefabName,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::cloned::Cloned<std::collections::hash::set::Iter<asset::prefab::PrefabName> > > > > > > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<asset::prefab::PrefabName,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$28::fmt<ref$<asset::prefab::PrefabName>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$28::fmt", "arguments": ["ref$<asset::prefab::PrefabName>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>", "constructor": "std::collections::hash::map::Keys", "arguments": ["asset::prefab::PrefabName", "country_core::resources::prefab::Prefab"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<asset::prefab::PrefabName>,std::collections::hash::map::Keys<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<asset::prefab::PrefabName>", "std::collections::hash::map::Keys<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "constructor": "std::collections::hash::map::Iter", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<asset::prefab::PrefabName,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["asset::prefab::PrefabName", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["asset::prefab::PrefabName", "country_core::resources::prefab::Prefab", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["asset::prefab::PrefabName", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<asset::prefab::PrefabName> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> >", "constructor": "country_core::resources::prefab::PrefabLibrary::get", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::prefab::impl$0::get::closure$0<ref$<asset::prefab::PrefabName> >", "constructor": "country_core::resources::prefab::impl$0::get::closure$0", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> >", "constructor": "country_core::resources::prefab::PrefabLibrary::has", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<ref$<asset::prefab::PrefabName> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["ref$<asset::prefab::PrefabName>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["std::path::PathBuf", "alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["std::path::PathBuf", "alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<asset::prefab::PrefabName,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["asset::prefab::PrefabName", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::PrefabNameRef
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ref$<asset::prefab::PrefabNameRef>", "constructor": "ref$", "arguments": ["asset::prefab::PrefabNameRef"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<ref$<asset::prefab::PrefabNameRef> >", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name", "arguments": ["ref$<asset::prefab::PrefabNameRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> >", "constructor": "country_core::resources::prefab::PrefabLibrary::has", "arguments": ["ref$<asset::prefab::PrefabNameRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<asset::prefab::PrefabNameRef>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["asset::prefab::PrefabNameRef"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: prefab.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: prefab.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 56; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: asset::prefab::PrefabName::as_name_ref
// 信息 ID: 61c7d273e1e16c23224ff879
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90cf46 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90d091 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fbf6 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fc84 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2> @ 0xa2dd5c -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2> @ 0xa2de0a -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name<ref$<asset::prefab::PrefabName> > @ 0xc04e3d -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> > @ 0xc05e79 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::impl$1::from @ 0xc06f85 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::next<tuple$<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0> @ 0x15e8acc -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> > @ 0x1819019 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> > @ 0x18190ac -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> > @ 0x1819168 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> > @ 0x18191ed -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> > @ 0x1819310 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_delete_tree::ui_delete_tree @ 0x20960fd -> 0x181c960 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::PrefabName::new
// 信息 ID: 4d13d5c79f90ef9db0aa6e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba8a86 -> 0x181c970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees @ 0xbb3922 -> 0x181c970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x285eedf -> 0x181c970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x2861002 -> 0x181c970 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_move_tree::ui_move_tree @ 0x2862ed9 -> 0x181c970 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=1805, inline_record=16860; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$10::eq
// 信息 ID: 49a1466bbc5e5c8369733c6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::prefab::hot_reload_prefabs; module=504, inline_record=46756; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap; module=539, inline_record=106604; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::reload_mesh_atlases; module=544, inline_record=25724; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<asset::prefab::PrefabName,country_core::resources::prefab::Prefab,std::hash::random::RandomState,alloc::alloc::Global>; module=1658, inline_record=232016; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>; module=1658, inline_record=381956; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState,alloc::alloc::Global>; module=3380, inline_record=3068; 不是独立调用边。
// [PDB 内联宿主] system_tree::ui_move_tree::ui_move_tree; module=3900, inline_record=3324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$12::hash
// 信息 ID: f4e95111d162e284ed4b8ad0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<asset::prefab::PrefabName> >; module=1660, inline_record=13812; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<asset::prefab::PrefabName> >; module=3378, inline_record=3344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$13::fmt
// 信息 ID: 1bfc6f74d12914f67e090c0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<ref$<asset::prefab::PrefabName> >; module=1685, inline_record=36940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$15::eq
// 信息 ID: d699d7adf63522e3a2c0c5ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>; module=533, inline_record=152748; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> >; module=1683, inline_record=40948; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> >; module=1683, inline_record=43288; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> >; module=1683, inline_record=44640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$18::fmt
// 信息 ID: 3452b6d37696d8b29477008f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<asset::prefab::PrefabNameRef>; module=1685, inline_record=36744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$4::as_ref
// 信息 ID: c9539b72dd71e44bc25efb3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 206 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2741 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x15092b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150952 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150bcc -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cd4 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cfe -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x14fde0 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150e75 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1879d5 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1875c2 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1878ea -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3be3 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3bf9 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3cb6 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x22453b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224c02 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$5::as_ref
// 信息 ID: 4d3708184f6faf8060487b86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90cf46 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::spawn_prefabs @ 0x90d091 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fbf6 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::prefab::hot_reload_prefabs @ 0x90fc84 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2> @ 0xa2dd5c -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2> @ 0xa2de0a -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name<ref$<asset::prefab::PrefabName> > @ 0xc04e3d -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> > @ 0xc05e79 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::impl$1::from @ 0xc06f85 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::next<tuple$<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0> @ 0x15e8acc -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> > @ 0x1819019 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> > @ 0x18190ac -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> > @ 0x1819168 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> > @ 0x18191ed -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> > @ 0x1819310 -> 0x181c960 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_delete_tree::ui_delete_tree @ 0x20960fd -> 0x181c960 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] asset::prefab::PrefabName::as_name_ref; module=1684, inline_record=660; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$6::borrow
// 信息 ID: c14647e74eadd92208140229
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>; module=533, inline_record=152696; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> >; module=1683, inline_record=40896; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabName> >; module=1683, inline_record=43236; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::prefab::PrefabLibrary::has<ref$<asset::prefab::PrefabNameRef> >; module=1683, inline_record=44588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::impl$8::clone
// 信息 ID: 289325cafefc8c19a5f183e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8215b979ec66a53246aa1991: asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>)
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::prefab::spawn_prefabs; module=504, inline_record=13292; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::impl$1::watch_prefab::closure$0; module=534, inline_record=39484; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=43472; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=539, inline_record=91268; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=539, inline_record=91616; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap; module=539, inline_record=107312; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=6228; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::reload_mesh_atlases; module=544, inline_record=21108; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
