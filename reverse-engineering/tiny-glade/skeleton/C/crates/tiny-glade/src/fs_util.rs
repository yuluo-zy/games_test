// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/fs_util.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e41caa9bd10f549f0020ee2b6c793e792c7ed35f6a2e554d95f1c9131c33e7ae
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fs_util.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 128, 235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":16652,"rva":4209344,"va":5372918464}
// 编译实例 RVAs: 0x403ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11771045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8a59cb300","size":5,"soft_fail":false,"target_va":5384689616,"va":5372918566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11770620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8fc9ab300","size":5,"soft_fail":false,"target_va":5384689584,"va":5372918959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11770441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8499ab300","size":5,"soft_fail":false,"target_va":5384689312,"va":5372918866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11849508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e824cfb400","size":5,"soft_fail":false,"target_va":5384768304,"va":5372918791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11737231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e88f18b300","size":5,"soft_fail":false,"target_va":5384656240,"va":5372919004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11845083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8dbbdb400","size":5,"soft_fail":false,"target_va":5384764464,"va":5372919376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t2759664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8f01b2a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372919035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3585432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8684ac9ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5372920435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fabe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2060351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e83f701f00","size":5,"soft_fail":false,"target_va":5374979040,"va":5372918684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fb240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2061413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e865741f00","size":5,"soft_fail":false,"target_va":5374980672,"va":5372919254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43403271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e807489602","size":5,"soft_fail":false,"target_va":5416321856,"va":5372918580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44138391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8977fa102","size":5,"soft_fail":false,"target_va":5417057056,"va":5372918660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t9203447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8f76e8c00","size":5,"soft_fail":false,"target_va":5382122800,"va":5372919348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11804488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e8481fb400","size":5,"soft_fail":false,"target_va":5384723728,"va":5372919235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11806540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e84c27b400","size":5,"soft_fail":false,"target_va":5384725152,"va":5372918607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372918464,"raw_hex":"e87ef5ffff","size":5,"soft_fail":false,"target_va":5372916368,"va":5372919053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":128,"record_offset":16852,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":18812,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":17084,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":16992,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":17152,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":17240,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":18728,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":16880,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":17020,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":128,"record_offset":17296,"parent_record_offset":16652,"ancestor_record_offset":16652,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":17552,"parent_record_offset":17528,"ancestor_record_offset":16652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":17576,"parent_record_offset":17552,"ancestor_record_offset":16652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":17600,"parent_record_offset":17576,"ancestor_record_offset":16652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":17528,"parent_record_offset":17472,"ancestor_record_offset":16652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":16652,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":17472,"parent_record_offset":17448,"ancestor_record_offset":16652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":16652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":19656,"parent_record_offset":19620,"ancestor_record_offset":16652,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":16652,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":128,"record_offset":16932,"parent_record_offset":16880,"ancestor_record_offset":16652,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=81
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":17328,"parent_record_offset":17296,"ancestor_record_offset":16652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":128,"record_offset":19688,"parent_record_offset":19656,"ancestor_record_offset":16652,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":17044,"parent_record_offset":17020,"ancestor_record_offset":16652,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":19716,"parent_record_offset":19656,"ancestor_record_offset":16652,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":19748,"parent_record_offset":19620,"ancestor_record_offset":16652,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":16960,"parent_record_offset":16880,"ancestor_record_offset":16652,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":16652,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":16652,"inlinee":13256,"source_line":181,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":16652,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":16652,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":8244,"rva":4207248,"va":5372916368}
// 编译实例 RVAs: 0x403290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11773127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e8c7a4b300","size":5,"soft_fail":false,"target_va":5384689616,"va":5372916484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11772700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e81ca3b300","size":5,"soft_fail":false,"target_va":5384689584,"va":5372916879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11772521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e869a2b300","size":5,"soft_fail":false,"target_va":5384689312,"va":5372916786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11851588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e844d7b400","size":5,"soft_fail":false,"target_va":5384768304,"va":5372916711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11739311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e8af20b300","size":5,"soft_fail":false,"target_va":5384656240,"va":5372916924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11847163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e8fbc5b400","size":5,"soft_fail":false,"target_va":5384764464,"va":5372917296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t2761744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e810242a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372916955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3583432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e83852c9ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5372918435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fabe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2062429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e85d781f00","size":5,"soft_fail":false,"target_va":5374979040,"va":5372916606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fb240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2063493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e8857c1f00","size":5,"soft_fail":false,"target_va":5374980672,"va":5372917174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43405353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e829509602","size":5,"soft_fail":false,"target_va":5416321856,"va":5372916498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44140469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e8b587a102","size":5,"soft_fail":false,"target_va":5417057056,"va":5372916582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t9205527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e817778c00","size":5,"soft_fail":false,"target_va":5382122800,"va":5372917268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11806568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e86827b400","size":5,"soft_fail":false,"target_va":5384723728,"va":5372917155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11808618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e86a2fb400","size":5,"soft_fail":false,"target_va":5384725152,"va":5372916529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372916368,"raw_hex":"e89efdffff","size":5,"soft_fail":false,"target_va":5372916368,"va":5372916973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":10364,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":8636,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":8544,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8704,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8792,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":10280,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8432,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8572,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":128,"record_offset":8848,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":8244,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":8244,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":8244,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":9080,"parent_record_offset":9024,"ancestor_record_offset":8244,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":9052,"parent_record_offset":9024,"ancestor_record_offset":8244,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":8244,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":128,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":8244,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":11208,"parent_record_offset":11172,"ancestor_record_offset":8244,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":128,"record_offset":11172,"parent_record_offset":11148,"ancestor_record_offset":8244,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":128,"record_offset":8484,"parent_record_offset":8432,"ancestor_record_offset":8244,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=101
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":8880,"parent_record_offset":8848,"ancestor_record_offset":8244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":128,"record_offset":11240,"parent_record_offset":11208,"ancestor_record_offset":8244,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":128,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":8244,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":11268,"parent_record_offset":11208,"ancestor_record_offset":8244,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":128,"record_offset":11300,"parent_record_offset":11172,"ancestor_record_offset":8244,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8512,"parent_record_offset":8432,"ancestor_record_offset":8244,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":128,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":8244,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":8244,"inlinee":13256,"source_line":181,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":8244,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":128,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":8244,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43BD
// 首条位置: {"module":235,"inlinee":17341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":20872,"parent_record_offset":20852,"ancestor_record_offset":20852,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":20852,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":20852,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43C2
// 首条位置: {"module":235,"inlinee":17346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":24132,"parent_record_offset":24112,"ancestor_record_offset":24112,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":235,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":24112,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00235.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":235,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":24112,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00235.jsonl.gz"}
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
// 完整结构及原始类型记录: fs_util.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fs_util.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: fs_util.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>", "country_core::systems::steam::SteamOverlayOpenResources", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<alloc::sync::Arc<egui::style::Style,alloc::alloc::Global> > > >

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$0
// 信息 ID: 620ee12872221fd902a2b11e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<ref$<std::path::PathBuf>,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<ref$<std::path::PathBuf>,std::path::PathBuf> >; module=235, inline_record=20852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$1
// 信息 ID: ec3d1cfb0a008fa6dd68369c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$1 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; module=235, inline_record=24112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
// 信息 ID: 640cf8c178a387739f6ed456
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["ref$<std::path::PathBuf>", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x422c19 -> 0x403ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: 31e33e7c0f3d9c4668bafcf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf> @ 0x4034ed -> 0x403290 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf> @ 0x403d0d -> 0x403290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
