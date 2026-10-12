// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/save_backup_list.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f697cad8b95bc5f93e7e61507cd85f0a279f0d56e2a5e3280cdc0dd3298550b2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_backup_list.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 558, 561, 564, 579, 1643, 1666, 1800, 2927
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_auto
// 记录形式: inlinee_declaration; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8873
// 首条位置: {"module":558,"inlinee":34931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":30500,"rva":11558176,"va":5380267296}
// 编译实例 RVAs: 0xb05d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$2::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":268,"rva":12267136,"va":5380976256}
// 编译实例 RVAs: 0xbb2e80, 0x1b6de10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3841358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380976256,"raw_hex":"e84e9d3a00","size":5,"soft_fail":false,"target_va":5384817664,"va":5380976301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9005
// 首条位置: {"module":579,"inlinee":36869}
// 编译实例 RVAs: 0xbb5420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3673297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380985888,"raw_hex":"e8d10c3800","size":5,"soft_fail":false,"target_va":5384659264,"va":5380985962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11647346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380985888,"raw_hex":"e88e464eff","size":5,"soft_fail":false,"target_va":5369338848,"va":5380986189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35780337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380985888,"raw_hex":"e8f1f62102","size":5,"soft_fail":false,"target_va":5416766384,"va":5380986042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36070971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380985888,"raw_hex":"e83b662602","size":5,"soft_fail":false,"target_va":5417057056,"va":5380986080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3739222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380985888,"raw_hex":"e8560e3900","size":5,"soft_fail":false,"target_va":5384725152,"va":5380985925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":579,"record_offset":24504,"parent_record_offset":24388,"ancestor_record_offset":24388,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":24428,"parent_record_offset":24388,"ancestor_record_offset":24388,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":579,"record_offset":24576,"parent_record_offset":24536,"ancestor_record_offset":24388,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":579,"record_offset":24536,"parent_record_offset":24504,"ancestor_record_offset":24388,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":24460,"parent_record_offset":24428,"ancestor_record_offset":24388,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D11
// 首条位置: {"module":1643,"inlinee":81169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":69064,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":68788,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1643,"record_offset":70356,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1643,"record_offset":69228,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1643,"record_offset":68560,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_file
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1643,"record_offset":68672,"parent_record_offset":68440,"ancestor_record_offset":68440,"inlinee":59717,"source_line":2007,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":69592,"parent_record_offset":69568,"ancestor_record_offset":68440,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":69616,"parent_record_offset":69592,"ancestor_record_offset":68440,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":69640,"parent_record_offset":69616,"ancestor_record_offset":68440,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":69568,"parent_record_offset":69512,"ancestor_record_offset":68440,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":69540,"parent_record_offset":69512,"ancestor_record_offset":68440,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":69512,"parent_record_offset":69488,"ancestor_record_offset":68440,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":69488,"parent_record_offset":69464,"ancestor_record_offset":68440,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":71484,"parent_record_offset":71448,"ancestor_record_offset":68440,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":68872,"parent_record_offset":68844,"ancestor_record_offset":68440,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":69252,"parent_record_offset":69228,"ancestor_record_offset":68440,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1643,"record_offset":68608,"parent_record_offset":68584,"ancestor_record_offset":68440,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":68816,"parent_record_offset":68788,"ancestor_record_offset":68440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":68988,"parent_record_offset":68912,"ancestor_record_offset":68440,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":69276,"parent_record_offset":69252,"ancestor_record_offset":68440,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1643,"record_offset":69304,"parent_record_offset":69276,"ancestor_record_offset":68440,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":70412,"parent_record_offset":70384,"ancestor_record_offset":68440,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1643,"record_offset":70384,"parent_record_offset":70356,"ancestor_record_offset":68440,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":68912,"parent_record_offset":68872,"ancestor_record_offset":68440,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":68956,"parent_record_offset":68912,"ancestor_record_offset":68440,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":69020,"parent_record_offset":68872,"ancestor_record_offset":68440,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1643,"record_offset":69180,"parent_record_offset":69152,"ancestor_record_offset":68440,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1643,"record_offset":69096,"parent_record_offset":69064,"ancestor_record_offset":68440,"inlinee":81171,"source_line":68,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1643,"record_offset":70224,"parent_record_offset":70192,"ancestor_record_offset":68440,"inlinee":81172,"source_line":75,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1643,"record_offset":69124,"parent_record_offset":69096,"ancestor_record_offset":68440,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::file_type
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1643,"record_offset":68696,"parent_record_offset":68672,"ancestor_record_offset":68440,"inlinee":18118,"source_line":1955,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1643,"record_offset":68584,"parent_record_offset":68560,"ancestor_record_offset":68440,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileAttr::file_type
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1643,"record_offset":68720,"parent_record_offset":68696,"ancestor_record_offset":68440,"inlinee":18120,"source_line":1088,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1643,"record_offset":68744,"parent_record_offset":68720,"ancestor_record_offset":68440,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::impl$24::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1643,"record_offset":68632,"parent_record_offset":68608,"ancestor_record_offset":68440,"inlinee":18115,"source_line":1133,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1643,"record_offset":69152,"parent_record_offset":69124,"ancestor_record_offset":68440,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D13
// 首条位置: {"module":1643,"inlinee":81171}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1643,"record_offset":69124,"parent_record_offset":69096,"ancestor_record_offset":69096,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1643,"record_offset":69180,"parent_record_offset":69152,"ancestor_record_offset":69096,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1643,"record_offset":69152,"parent_record_offset":69124,"ancestor_record_offset":69096,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D14
// 首条位置: {"module":1643,"inlinee":81172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1643,"record_offset":70252,"parent_record_offset":70224,"ancestor_record_offset":70224,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1643,"record_offset":70308,"parent_record_offset":70280,"ancestor_record_offset":70224,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1643,"record_offset":70280,"parent_record_offset":70252,"ancestor_record_offset":70224,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::new_from_session
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":24136,"rva":12273792,"va":5380982912}
// 编译实例 RVAs: 0xbb4880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3676155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8fb173800","size":5,"soft_fail":false,"target_va":5384659264,"va":5380983104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3706144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8208d3800","size":5,"soft_fail":false,"target_va":5384689616,"va":5380983467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3784424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8e8be3900","size":5,"soft_fail":false,"target_va":5384768304,"va":5380983875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3673178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e85a0c3800","size":5,"soft_fail":false,"target_va":5384657312,"va":5380984129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3673369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8190d3800","size":5,"soft_fail":false,"target_va":5384656400,"va":5380983026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3779726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e88eac3900","size":5,"soft_fail":false,"target_va":5384764464,"va":5380984733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3625982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8fe533700","size":5,"soft_fail":false,"target_va":5384609408,"va":5380983421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5305659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8c50aafff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380984358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11715565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8133c4dff","size":5,"soft_fail":false,"target_va":5369270288,"va":5380985848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11644868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e83c504eff","size":5,"soft_fail":false,"target_va":5369338848,"va":5380983711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140989630
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2275748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e85c46ddff","size":5,"soft_fail":false,"target_va":5378709040,"va":5380984783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417cf330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12691057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e871a6c100","size":5,"soft_fail":false,"target_va":5393675056,"va":5380983994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35783171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e803022202","size":5,"soft_fail":false,"target_va":5416766384,"va":5380983208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36072721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8116d2602","size":5,"soft_fail":false,"target_va":5417057040,"va":5380984314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36073309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e85d6f2602","size":5,"soft_fail":false,"target_va":5417057056,"va":5380983742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3810611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e833253a00","size":5,"soft_fail":false,"target_va":5384794864,"va":5380984248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35383721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8a9e91b02","size":5,"soft_fail":false,"target_va":5416368112,"va":5380984386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t1138753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e841601100","size":5,"soft_fail":false,"target_va":5382122800,"va":5380984042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3640641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8418d3700","size":5,"soft_fail":false,"target_va":5384624080,"va":5380983434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35636702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e8dec51f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380984893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3845915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e81baf3a00","size":5,"soft_fail":false,"target_va":5384830080,"va":5380984160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-715229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e82316f5ff","size":5,"soft_fail":false,"target_va":5380267776,"va":5380983000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3742094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380982912,"raw_hex":"e88e193900","size":5,"soft_fail":false,"target_va":5384725152,"va":5380983053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":579,"record_offset":27288,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":24360,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":27908,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":28948,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":24704,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":579,"record_offset":27372,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":579,"record_offset":25264,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":27252,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":24872,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":579,"record_offset":28504,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":6807,"source_line":2765,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":579,"record_offset":27400,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":15542,"source_line":2443,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":579,"record_offset":24388,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":36869,"source_line":54,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":24760,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":24620,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":579,"record_offset":27168,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":24928,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":27800,"parent_record_offset":27776,"ancestor_record_offset":24136,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":24136,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":27848,"parent_record_offset":27824,"ancestor_record_offset":24136,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":27776,"parent_record_offset":27720,"ancestor_record_offset":24136,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":579,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":24136,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":28012,"parent_record_offset":27980,"ancestor_record_offset":24136,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":24136,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":27720,"parent_record_offset":27696,"ancestor_record_offset":24136,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":28060,"parent_record_offset":28036,"ancestor_record_offset":24136,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":24136,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":27696,"parent_record_offset":27672,"ancestor_record_offset":24136,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$26::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":579,"record_offset":27424,"parent_record_offset":27400,"ancestor_record_offset":24136,"inlinee":30755,"source_line":2635,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":26816,"parent_record_offset":26780,"ancestor_record_offset":24136,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":24136,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":27980,"parent_record_offset":27936,"ancestor_record_offset":24136,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":24136,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":579,"record_offset":24576,"parent_record_offset":24536,"ancestor_record_offset":24136,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":579,"record_offset":24536,"parent_record_offset":24504,"ancestor_record_offset":24136,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":579,"record_offset":24504,"parent_record_offset":24388,"ancestor_record_offset":24136,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":579,"record_offset":28696,"parent_record_offset":28588,"ancestor_record_offset":24136,"inlinee":6771,"source_line":402,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":27520,"parent_record_offset":27496,"ancestor_record_offset":24136,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":579,"record_offset":28812,"parent_record_offset":28756,"ancestor_record_offset":24136,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$11::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":579,"record_offset":25344,"parent_record_offset":25304,"ancestor_record_offset":24136,"inlinee":33987,"source_line":235,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$46::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":579,"record_offset":25384,"parent_record_offset":25344,"ancestor_record_offset":24136,"inlinee":36870,"source_line":871,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::try_flatten_one::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":579,"record_offset":25556,"parent_record_offset":25520,"ancestor_record_offset":24136,"inlinee":36873,"source_line":816,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$10::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":579,"record_offset":25464,"parent_record_offset":25424,"ancestor_record_offset":24136,"inlinee":36871,"source_line":310,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":579,"record_offset":25424,"parent_record_offset":25384,"ancestor_record_offset":24136,"inlinee":33990,"source_line":83,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":579,"record_offset":25304,"parent_record_offset":25264,"ancestor_record_offset":24136,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":579,"record_offset":25784,"parent_record_offset":25556,"ancestor_record_offset":24136,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":579,"record_offset":25520,"parent_record_offset":25464,"ancestor_record_offset":24136,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":579,"record_offset":25748,"parent_record_offset":25724,"ancestor_record_offset":24136,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":579,"record_offset":28668,"parent_record_offset":28588,"ancestor_record_offset":24136,"inlinee":6770,"source_line":1521,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$38::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":579,"record_offset":28532,"parent_record_offset":28504,"ancestor_record_offset":24136,"inlinee":13727,"source_line":1578,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$39::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":579,"record_offset":28588,"parent_record_offset":28560,"ancestor_record_offset":24136,"inlinee":13730,"source_line":1708,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$39::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":579,"record_offset":28560,"parent_record_offset":28532,"ancestor_record_offset":24136,"inlinee":13729,"source_line":1629,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":28756,"parent_record_offset":28588,"ancestor_record_offset":24136,"inlinee":13733,"source_line":1160,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":28784,"parent_record_offset":28756,"ancestor_record_offset":24136,"inlinee":13734,"source_line":3050,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":28724,"parent_record_offset":28696,"ancestor_record_offset":24136,"inlinee":4145,"source_line":2493,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":24728,"parent_record_offset":24704,"ancestor_record_offset":24136,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":27312,"parent_record_offset":27288,"ancestor_record_offset":24136,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":25724,"parent_record_offset":25700,"ancestor_record_offset":24136,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":24896,"parent_record_offset":24872,"ancestor_record_offset":24136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":26848,"parent_record_offset":26816,"ancestor_record_offset":24136,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":28100,"parent_record_offset":27936,"ancestor_record_offset":24136,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":24460,"parent_record_offset":24428,"ancestor_record_offset":24136,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":25632,"parent_record_offset":25604,"ancestor_record_offset":24136,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":25700,"parent_record_offset":25556,"ancestor_record_offset":24136,"inlinee":33993,"source_line":2079,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":25604,"parent_record_offset":25556,"ancestor_record_offset":24136,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":579,"record_offset":27544,"parent_record_offset":27520,"ancestor_record_offset":24136,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":579,"record_offset":27568,"parent_record_offset":27544,"ancestor_record_offset":24136,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":579,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":24136,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":579,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":24136,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":579,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":24136,"inlinee":15545,"source_line":1017,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":25972,"parent_record_offset":25936,"ancestor_record_offset":24136,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":24644,"parent_record_offset":24620,"ancestor_record_offset":24136,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":24668,"parent_record_offset":24644,"ancestor_record_offset":24136,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":25996,"parent_record_offset":25972,"ancestor_record_offset":24136,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":26028,"parent_record_offset":25936,"ancestor_record_offset":24136,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":579,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":24136,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":24136,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":24784,"parent_record_offset":24760,"ancestor_record_offset":24136,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":24832,"parent_record_offset":24808,"ancestor_record_offset":24136,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":24980,"parent_record_offset":24928,"ancestor_record_offset":24136,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":24952,"parent_record_offset":24928,"ancestor_record_offset":24136,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":24428,"parent_record_offset":24388,"ancestor_record_offset":24136,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":579,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":24136,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::into_files
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9040
// 首条位置: {"module":1800,"inlinee":36928}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::backup_metronome_counter
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89C3
// 首条位置: {"module":561,"inlinee":35267}
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
// 完整结构及原始类型记录: save_backup_list.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_backup_list::BackupSaveFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile>", "constructor": "core::option::Option", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_backup_list::BackupSaveFile,enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>", "country_core::systems::save::save_backup_list::BackupSaveFile", "enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "toolbar::menu::save_error::ui::closure$4::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "toolbar::menu::load::ui::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_backup_list::BackupSaveFileKind
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_backup_list::SaveBackupList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::save_backup_list::SaveBackupList>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::save_backup_list::SaveBackupList"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: save_backup_list.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_backup_list.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_auto
// 信息 ID: 607cbe7cee5740923333c23f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted
// 信息 ID: 3dae6bce4605c90d61537ec7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56a875 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::fill_weather_slots @ 0x6736bc -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$12::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x6859de -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$13,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x687dda -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0x116f057 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a0479 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x285ed03 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x2860f5f -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x2860f86 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x28cfbc5 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x28cfcc5 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x28cfce8 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x28cfa29 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing @ 0x28cffa9 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0x2938e93 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_unstuck::sheep_unstuck @ 0x2943ab6 -> 0xb05d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::backup_metronome_counter
// 信息 ID: 8b33142592e3aeae1bf8b4bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=564, inline_record=51444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::into_files
// 信息 ID: f8b47296a24835c22f0b3fc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=94796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
// 信息 ID: e9fb97889f9267bedfa5b994
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::save_error::ui::closure$4 @ 0x119bdbe -> 0xbb5420 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x2348ce9 -> 0xbb5420 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::save::save_backup_list::SaveBackupList::new_from_session; module=579, inline_record=24388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=50604; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::new_from_session
// 信息 ID: 679da5533a75a2f921680cc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0xb273c7 -> 0xbb4880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0
// 信息 ID: 450a69d8018a3ad45593891e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c81adc9aa50eea9209da5a65: enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_backup_list::impl$1::iter_backup_files", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_backup_list::BackupSaveFile,enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_backup_list::BackupSaveFile,enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 10 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1643, inline_record=68440; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1666, inline_record=73340; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2927, inline_record=508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
// 信息 ID: 30085c248b6911ef7ef5c558
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_backup_list::impl$1::iter_backup_files", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1643, inline_record=69096; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1666, inline_record=74056; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2927, inline_record=1224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
// 信息 ID: f3d043cbf0d20e07a94c45e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_backup_list::impl$1::iter_backup_files", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1643, inline_record=70224; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1666, inline_record=75180; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2927, inline_record=2348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$2::fmt
// 信息 ID: 7a192a9f5c788ddf4c1048d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
