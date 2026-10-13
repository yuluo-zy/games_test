// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/motomoto/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d96b99ab372377819a35fc0603565521f1a4e41aa0e4baaaab7cedbfb3760b31
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 515, 524, 1253, 1694, 1806, 1812, 2917
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use motomoto; // namespace candidate
//   package identities: [{"name":"motomoto","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: motomoto::impl$0::as_tag
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":264,"rva":832880,"va":5369542000}
// 编译实例 RVAs: 0xcb570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$1::as_tag
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":364,"rva":37170672,"va":5405879792}
// 编译实例 RVAs: 0x2372df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11177223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405879792,"raw_hex":"e8078daa00","size":5,"soft_fail":false,"target_va":5417057056,"va":5405879828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10742334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405879792,"raw_hex":"e83eeaa300","size":5,"soft_fail":false,"target_va":5416622208,"va":5405879869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] motomoto::tag_from_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":2917,"record_offset":472,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":111886,"source_line":63,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":2917,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":364,"inlinee":111887,"source_line":47,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_le_bytes_truncated
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B50F
// 首条位置: {"module":2917,"inlinee":111887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_str
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B50E
// 首条位置: {"module":2917,"inlinee":111886}
// 编译实例 RVAs: 0x2372f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11176904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405880112,"raw_hex":"e8c88baa00","size":5,"soft_fail":false,"target_va":5417057056,"va":5405880147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10742015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405880112,"raw_hex":"e8ffe8a300","size":5,"soft_fail":false,"target_va":5416622208,"va":5405880188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":2917,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":472,"inlinee":111887,"source_line":47,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_str_trunc
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":1908,"rva":37171088,"va":5405880208}
// 编译实例 RVAs: 0x2372f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11176800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405880208,"raw_hex":"e8608baa00","size":5,"soft_fail":false,"target_va":5417057056,"va":5405880251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":2917,"record_offset":2020,"parent_record_offset":1908,"ancestor_record_offset":1908,"inlinee":111887,"source_line":47,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1132D
// 首条位置: {"module":1806,"inlinee":70445}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1253,"record_offset":46180,"rva":18551776,"va":5387260896}
// 编译实例 RVAs: 0x11b13e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142372df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18618820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387260896,"raw_hex":"e8c4191c01","size":5,"soft_fail":false,"target_va":5405879792,"va":5387260967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29091809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387260896,"raw_hex":"e8e1e7bb01","size":5,"soft_fail":false,"target_va":5416353040,"va":5387261226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29796042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387260896,"raw_hex":"e8caa6c601","size":5,"soft_fail":false,"target_va":5417057088,"va":5387261041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1253,"record_offset":46448,"parent_record_offset":46180,"ancestor_record_offset":46180,"inlinee":9141,"source_line":1367,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1253,"record_offset":46392,"parent_record_offset":46180,"ancestor_record_offset":46180,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1253,"record_offset":46420,"parent_record_offset":46180,"ancestor_record_offset":46180,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":46656,"parent_record_offset":46632,"ancestor_record_offset":46180,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":46772,"parent_record_offset":46744,"ancestor_record_offset":46180,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":46680,"parent_record_offset":46656,"ancestor_record_offset":46180,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":46832,"parent_record_offset":46800,"ancestor_record_offset":46180,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":46800,"parent_record_offset":46772,"ancestor_record_offset":46180,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":46632,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":46716,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":46744,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":47044,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":46932,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1253,"record_offset":46604,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1253,"record_offset":46856,"parent_record_offset":46832,"ancestor_record_offset":46180,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1253,"record_offset":47124,"parent_record_offset":47100,"ancestor_record_offset":46180,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1253,"record_offset":46956,"parent_record_offset":46932,"ancestor_record_offset":46180,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1253,"record_offset":47100,"parent_record_offset":47076,"ancestor_record_offset":46180,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1253,"record_offset":46988,"parent_record_offset":46548,"ancestor_record_offset":46180,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1253,"record_offset":47012,"parent_record_offset":46988,"ancestor_record_offset":46180,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$8::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":46476,"parent_record_offset":46448,"ancestor_record_offset":46180,"inlinee":9133,"source_line":640,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":46548,"parent_record_offset":46504,"ancestor_record_offset":46180,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":46504,"parent_record_offset":46476,"ancestor_record_offset":46180,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":47076,"parent_record_offset":46504,"ancestor_record_offset":46180,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1253,"record_offset":47956,"rva":18552176,"va":5387261296}
// 编译实例 RVAs: 0x11b1570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17719367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387261296,"raw_hex":"e8b99ff1fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5387261362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29091414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387261296,"raw_hex":"e856e6bb01","size":5,"soft_fail":false,"target_va":5416353040,"va":5387261621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29795647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387261296,"raw_hex":"e83fa5c601","size":5,"soft_fail":false,"target_va":5417057088,"va":5387261436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1253,"record_offset":48216,"parent_record_offset":47956,"ancestor_record_offset":47956,"inlinee":9141,"source_line":1367,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1253,"record_offset":48160,"parent_record_offset":47956,"ancestor_record_offset":47956,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1253,"record_offset":48188,"parent_record_offset":47956,"ancestor_record_offset":47956,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":48424,"parent_record_offset":48400,"ancestor_record_offset":47956,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":48540,"parent_record_offset":48512,"ancestor_record_offset":47956,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":48448,"parent_record_offset":48424,"ancestor_record_offset":47956,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":48600,"parent_record_offset":48568,"ancestor_record_offset":47956,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1253,"record_offset":48568,"parent_record_offset":48540,"ancestor_record_offset":47956,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":48400,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":48484,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":48512,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":48812,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1253,"record_offset":48700,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1253,"record_offset":48372,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1253,"record_offset":48624,"parent_record_offset":48600,"ancestor_record_offset":47956,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1253,"record_offset":48892,"parent_record_offset":48868,"ancestor_record_offset":47956,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1253,"record_offset":48724,"parent_record_offset":48700,"ancestor_record_offset":47956,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1253,"record_offset":48868,"parent_record_offset":48844,"ancestor_record_offset":47956,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1253,"record_offset":48756,"parent_record_offset":48316,"ancestor_record_offset":47956,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1253,"record_offset":48780,"parent_record_offset":48756,"ancestor_record_offset":47956,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$8::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":48244,"parent_record_offset":48216,"ancestor_record_offset":47956,"inlinee":9133,"source_line":640,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":48316,"parent_record_offset":48272,"ancestor_record_offset":47956,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":48272,"parent_record_offset":48244,"ancestor_record_offset":47956,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1253,"record_offset":48844,"parent_record_offset":48272,"ancestor_record_offset":47956,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$3::write
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x790B
// 首条位置: {"module":524,"inlinee":30987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::io::cursor::impl$8::write
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":524,"record_offset":50972,"parent_record_offset":50944,"ancestor_record_offset":50944,"inlinee":11939,"source_line":627,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":51148,"parent_record_offset":51124,"ancestor_record_offset":50944,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":51264,"parent_record_offset":51236,"ancestor_record_offset":50944,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":51172,"parent_record_offset":51148,"ancestor_record_offset":50944,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":51324,"parent_record_offset":51292,"ancestor_record_offset":50944,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":51292,"parent_record_offset":51264,"ancestor_record_offset":50944,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":51124,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=53
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":51208,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":51236,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":51536,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":51424,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":51096,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":51348,"parent_record_offset":51324,"ancestor_record_offset":50944,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":51620,"parent_record_offset":51592,"ancestor_record_offset":50944,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":51448,"parent_record_offset":51424,"ancestor_record_offset":50944,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1694,"record_offset":15384,"parent_record_offset":15356,"ancestor_record_offset":14720,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/01694.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":51480,"parent_record_offset":51044,"ancestor_record_offset":50944,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":524,"record_offset":51504,"parent_record_offset":51480,"ancestor_record_offset":50944,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":524,"record_offset":51044,"parent_record_offset":51000,"ancestor_record_offset":50944,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":524,"record_offset":51000,"parent_record_offset":50972,"ancestor_record_offset":50944,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":524,"record_offset":51568,"parent_record_offset":51000,"ancestor_record_offset":50944,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::ChunkWriter::seek_back_and_write_length
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7573
// 首条位置: {"module":515,"inlinee":30067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":515,"record_offset":263612,"parent_record_offset":263568,"ancestor_record_offset":263568,"inlinee":9141,"source_line":1367,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263812,"parent_record_offset":263788,"ancestor_record_offset":263568,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263928,"parent_record_offset":263900,"ancestor_record_offset":263568,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263836,"parent_record_offset":263812,"ancestor_record_offset":263568,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263984,"parent_record_offset":263956,"ancestor_record_offset":263568,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263956,"parent_record_offset":263928,"ancestor_record_offset":263568,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263788,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263872,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263900,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":264192,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":264080,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":263760,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":264008,"parent_record_offset":263984,"ancestor_record_offset":263568,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":264276,"parent_record_offset":264252,"ancestor_record_offset":263568,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":515,"record_offset":264104,"parent_record_offset":264080,"ancestor_record_offset":263568,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":515,"record_offset":264252,"parent_record_offset":264224,"ancestor_record_offset":263568,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":515,"record_offset":264136,"parent_record_offset":263712,"ancestor_record_offset":263568,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":515,"record_offset":264160,"parent_record_offset":264136,"ancestor_record_offset":263568,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$8::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263644,"parent_record_offset":263612,"ancestor_record_offset":263568,"inlinee":9133,"source_line":640,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263712,"parent_record_offset":263676,"ancestor_record_offset":263568,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263676,"parent_record_offset":263644,"ancestor_record_offset":263568,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":264224,"parent_record_offset":263676,"ancestor_record_offset":263568,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$5::drop
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7571
// 首条位置: {"module":515,"inlinee":30065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":264356,"parent_record_offset":263536,"ancestor_record_offset":263536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_direct] motomoto::ChunkWriter::seek_back_and_write_length
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":515,"record_offset":263568,"parent_record_offset":263536,"ancestor_record_offset":263536,"inlinee":30067,"source_line":114,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263812,"parent_record_offset":263788,"ancestor_record_offset":263536,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263928,"parent_record_offset":263900,"ancestor_record_offset":263536,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263836,"parent_record_offset":263812,"ancestor_record_offset":263536,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263984,"parent_record_offset":263956,"ancestor_record_offset":263536,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":263956,"parent_record_offset":263928,"ancestor_record_offset":263536,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263788,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263872,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":263900,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":264192,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":264080,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":515,"record_offset":263612,"parent_record_offset":263568,"ancestor_record_offset":263536,"inlinee":9141,"source_line":1367,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":263760,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":264008,"parent_record_offset":263984,"ancestor_record_offset":263536,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":264276,"parent_record_offset":264252,"ancestor_record_offset":263536,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":515,"record_offset":264104,"parent_record_offset":264080,"ancestor_record_offset":263536,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":515,"record_offset":264252,"parent_record_offset":264224,"ancestor_record_offset":263536,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":515,"record_offset":264136,"parent_record_offset":263712,"ancestor_record_offset":263536,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":515,"record_offset":264160,"parent_record_offset":264136,"ancestor_record_offset":263536,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$8::write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263644,"parent_record_offset":263612,"ancestor_record_offset":263536,"inlinee":9133,"source_line":640,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263712,"parent_record_offset":263676,"ancestor_record_offset":263536,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":263676,"parent_record_offset":263644,"ancestor_record_offset":263536,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":515,"record_offset":264224,"parent_record_offset":263676,"ancestor_record_offset":263536,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Reader::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":2064,"rva":7119936,"va":5375829056}
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
// 函数符号: motomoto::Reader::iter
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":2164,"rva":37171168,"va":5405880288}
// 编译实例 RVAs: 0x2372fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$9::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2917,"record_offset":544,"rva":37170768,"va":5405879888}
// 编译实例 RVAs: 0x2372e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10742211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405879888,"raw_hex":"e8c3e9a300","size":5,"soft_fail":false,"target_va":5416622320,"va":5405880104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::transpose
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2917,"record_offset":1724,"parent_record_offset":544,"ancestor_record_offset":544,"inlinee":76376,"source_line":1816,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_direct] motomoto::ChunkIterator::try_next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":2917,"record_offset":656,"parent_record_offset":544,"ancestor_record_offset":544,"inlinee":111889,"source_line":187,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2917,"record_offset":716,"parent_record_offset":656,"ancestor_record_offset":544,"inlinee":64037,"source_line":321,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2917,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":544,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2917,"record_offset":1556,"parent_record_offset":1460,"ancestor_record_offset":544,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1656,"parent_record_offset":1620,"ancestor_record_offset":544,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":544,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":544,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":544,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2917,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":544,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":544,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":820,"parent_record_offset":768,"ancestor_record_offset":544,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":544,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":544,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1684,"parent_record_offset":1620,"ancestor_record_offset":544,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1592,"parent_record_offset":656,"ancestor_record_offset":544,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":544,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":544,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":1432,"parent_record_offset":656,"ancestor_record_offset":544,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":544,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":2917,"record_offset":908,"parent_record_offset":740,"ancestor_record_offset":544,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::ChunkIterator::try_next
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B511
// 首条位置: {"module":2917,"inlinee":111889}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2917,"record_offset":716,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":64037,"source_line":321,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1592,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":1432,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2917,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":656,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2917,"record_offset":1556,"parent_record_offset":1460,"ancestor_record_offset":656,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1656,"parent_record_offset":1620,"ancestor_record_offset":656,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":656,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":656,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2917,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":656,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2917,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":656,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":656,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":656,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":820,"parent_record_offset":768,"ancestor_record_offset":656,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":656,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2917,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":656,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1684,"parent_record_offset":1620,"ancestor_record_offset":656,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2917,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":656,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":656,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2917,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":656,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/02917.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":2917,"record_offset":908,"parent_record_offset":740,"ancestor_record_offset":656,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/02917.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::ChunkIterator
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::ChunkWriter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "constructor": "motomoto::ChunkWriter", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::Serializer", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "core::ptr::drop_in_place", "arguments": ["serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "constructor": "ref_mut$", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >", "constructor": "flate2::deflate::write::DeflateEncoder", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::zio::Writer::write_with_status<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>", "constructor": "flate2::zio::Writer::write_with_status", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "flate2::mem::Compress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::zio::Writer::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>", "constructor": "flate2::zio::Writer::finish", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "flate2::mem::Compress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::io::Write::write_all<flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > > >", "constructor": "std::io::Write::write_all", "arguments": ["flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::deflate::write::DeflateEncoder::new<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >", "constructor": "flate2::deflate::write::DeflateEncoder::new", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::deflate::write::DeflateEncoder::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >", "constructor": "flate2::deflate::write::DeflateEncoder::finish", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "ref_mut$", "arguments": ["serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::Compound", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "enum2$", "arguments": ["serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,alloc::string::String,enum2$<serde_json::value::Value> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "alloc::string::String", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::format_escaped_str<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::format_escaped_str", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> > >", "constructor": "serde_json::value::ser::impl$0::serialize", "arguments": ["ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::number::impl$6::serialize<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> > >", "constructor": "serde_json::number::impl$6::serialize", "arguments": ["ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::Reader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::Writer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >", "constructor": "motomoto::Writer::add_chunk", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>", "constructor": "motomoto::Writer::add_chunk", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "u64"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 78; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: motomoto::ChunkIterator::try_next
// 信息 ID: a14c253dc67fdf30ce022bc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] motomoto::impl$9::next; module=2917, inline_record=656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::ChunkWriter::seek_back_and_write_length
// 信息 ID: f040f48520c508abb7bd827b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >; module=515, inline_record=263568; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >; module=515, inline_record=510148; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=62260; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=73544; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=76256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=78444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Reader::iter
// 信息 ID: d68b187a2da1a7e339a269f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfd648 -> 0x2372fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Reader::new
// 信息 ID: 4fc33109154da8e9a053ab92
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
// 函数: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >
// 信息 ID: ba40c08714e1b7b31e497349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] motomoto::Writer::add_chunk -> ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x1bf5c0e -> 0x11b13e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x1bf69d1 -> 0x11b13e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x1bf6f96 -> 0x11b13e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>
// 信息 ID: a162e4661d01c59fe66431e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] motomoto::Writer::add_chunk -> ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x1bf6652 -> 0x11b1570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Writer::new
// 信息 ID: 64f309caf8fde925c11a25a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=27184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$0::as_tag
// 信息 ID: ae95d781123401b615d5c3ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 388 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::buffer::Buffer> @ 0x9f196 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::sampler::Sampler> @ 0xa1750 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::texture::Texture> @ 0xa1a29 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::RasterPipeline> @ 0xa7c26 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSet> @ 0xa9c9f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::render_pass::RenderPass> @ 0xaa1a0 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::ComputePipeline> @ 0xaa470 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::swapchain::FallibleSwapchain> @ 0xaff6f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSetLayout> @ 0xb17d8 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186dcb -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186ebf -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f87 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x187051 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x222070 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label_with_id<ref$<str$> > @ 0x223560 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<ref$<str$> > @ 0x223a5c -> 0xcb570 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$1::as_tag
// 信息 ID: 14309b9a4925e0d76bc7548e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> > @ 0x11b1427 -> 0x2372df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$3::write
// 信息 ID: 569bf5ebb1baf2b056706640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] flate2::zio::Writer::write_with_status<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=524, inline_record=50944; 不是独立调用边。
// [PDB 内联宿主] flate2::zio::Writer::write_with_status<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=524, inline_record=51696; 不是独立调用边。
// [PDB 内联宿主] flate2::zio::Writer::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=524, inline_record=55580; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >; module=1694, inline_record=14720; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >; module=1694, inline_record=15588; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >; module=1694, inline_record=16648; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >; module=1694, inline_record=17568; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,alloc::string::String,enum2$<serde_json::value::Value> >; module=1694, inline_record=112288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$5::drop
// 信息 ID: 03ec842de597efc73a3b2a8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >; module=515, inline_record=263536; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >; module=515, inline_record=510116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=62224; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=73508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=76220; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1806, inline_record=78408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$9::next
// 信息 ID: 119743c071a51ac5805b6f38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfd65c -> 0x2372e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfecf1 -> 0x2372e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bff166 -> 0x2372e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > > @ 0x1c37717 -> 0x2372e50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_le_bytes_truncated
// 信息 ID: 939bd6662074cbf80194b29b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] motomoto::impl$1::as_tag; module=2917, inline_record=500; 不是独立调用边。
// [PDB 内联宿主] motomoto::tag_from_str; module=2917, inline_record=1868; 不是独立调用边。
// [PDB 内联宿主] motomoto::tag_from_str_trunc; module=2917, inline_record=2020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_str
// 信息 ID: f131537991b80803dc56a395
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bff1a3 -> 0x2372f30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] motomoto::impl$1::as_tag; module=2917, inline_record=472; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_str_trunc
// 信息 ID: 1089916cf4de8c77ca3a3b2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x1bf663f -> 0x2372f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
