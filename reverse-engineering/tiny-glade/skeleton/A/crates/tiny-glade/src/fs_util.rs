// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/fs_util.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e41caa9bd10f549f0020ee2b6c793e792c7ed35f6a2e554d95f1c9131c33e7ae
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fs_util.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 72, 134
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":35648,"rva":912848,"va":5369621968}
// 编译实例 RVAs: 0xdedd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7785764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e824cd7600","size":5,"soft_fail":false,"target_va":5377408080,"va":5369622311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7842519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8d7aa7700","size":5,"soft_fail":false,"target_va":5377465440,"va":5369622916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1591938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8824a1800","size":5,"soft_fail":false,"target_va":5371214128,"va":5369622185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1592038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8e64a1800","size":5,"soft_fail":false,"target_va":5371214848,"va":5369622805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2033800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e888081f00","size":5,"soft_fail":false,"target_va":5371656704,"va":5369622899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2685176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8f8f82800","size":5,"soft_fail":false,"target_va":5372309344,"va":5369624163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3127110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e846b72f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369622597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=8
//     target_aliases=28 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5549038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8eeab5400","size":5,"soft_fail":false,"target_va":5375171056,"va":5369622013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33971193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8f95b0602","size":5,"soft_fail":false,"target_va":5403593360,"va":5369622162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t33327822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8ce8afc01","size":5,"soft_fail":false,"target_va":5402949904,"va":5369622077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7786683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8bbd07600","size":5,"soft_fail":false,"target_va":5377408752,"va":5369622064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7786178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8c2ce7600","size":5,"soft_fail":false,"target_va":5377408720,"va":5369622537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7786275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e823cf7600","size":5,"soft_fail":false,"target_va":5377408688,"va":5369622408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7785829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e865cd7600","size":5,"soft_fail":false,"target_va":5377408224,"va":5369622390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7817204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8f4477700","size":5,"soft_fail":false,"target_va":5377439776,"va":5369622567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7852388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e864d17700","size":5,"soft_fail":false,"target_va":5377475184,"va":5369622791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7846108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e8dcb87700","size":5,"soft_fail":false,"target_va":5377468224,"va":5369622111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369621968,"raw_hex":"e874f4ffff","size":5,"soft_fail":false,"target_va":5369619664,"va":5369622615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":72,"record_offset":35848,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":39028,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":36420,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":36328,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":36524,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":38384,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":36044,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":36356,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":36776,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":36608,"parent_record_offset":35648,"ancestor_record_offset":35648,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":37164,"parent_record_offset":37140,"ancestor_record_offset":35648,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":37140,"parent_record_offset":37084,"ancestor_record_offset":35648,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":37112,"parent_record_offset":37084,"ancestor_record_offset":35648,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":37084,"parent_record_offset":37056,"ancestor_record_offset":35648,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":72,"record_offset":39736,"parent_record_offset":39700,"ancestor_record_offset":35648,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":72,"record_offset":39700,"parent_record_offset":39676,"ancestor_record_offset":35648,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":72,"record_offset":36096,"parent_record_offset":36044,"ancestor_record_offset":35648,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=94
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":36948,"parent_record_offset":36776,"ancestor_record_offset":35648,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":72,"record_offset":39768,"parent_record_offset":39736,"ancestor_record_offset":35648,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":36380,"parent_record_offset":36356,"ancestor_record_offset":35648,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":72,"record_offset":39796,"parent_record_offset":39736,"ancestor_record_offset":35648,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":72,"record_offset":39828,"parent_record_offset":39700,"ancestor_record_offset":35648,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":35944,"parent_record_offset":35920,"ancestor_record_offset":35648,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":35992,"parent_record_offset":35968,"ancestor_record_offset":35648,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":35648,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":36292,"parent_record_offset":36044,"ancestor_record_offset":35648,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35648,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":35920,"parent_record_offset":35896,"ancestor_record_offset":35648,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":35896,"parent_record_offset":35872,"ancestor_record_offset":35648,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":36808,"parent_record_offset":36776,"ancestor_record_offset":35648,"inlinee":8485,"source_line":3158,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":35872,"parent_record_offset":35848,"ancestor_record_offset":35648,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":72,"record_offset":36548,"parent_record_offset":36524,"ancestor_record_offset":35648,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":72,"record_offset":36572,"parent_record_offset":36548,"ancestor_record_offset":35648,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":26468,"rva":910544,"va":5369619664}
// 编译实例 RVAs: 0xde4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7788052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e814d67600","size":5,"soft_fail":false,"target_va":5377408080,"va":5369620023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7844807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8c7b37700","size":5,"soft_fail":false,"target_va":5377465440,"va":5369620628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1594235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e87b531800","size":5,"soft_fail":false,"target_va":5371214128,"va":5369619888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1594326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8d6531800","size":5,"soft_fail":false,"target_va":5371214848,"va":5369620517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2036088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e878111f00","size":5,"soft_fail":false,"target_va":5371656704,"va":5369620611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2687400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8a8012900","size":5,"soft_fail":false,"target_va":5372309344,"va":5369621939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3129398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e836c02f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369620309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=8
//     target_aliases=28 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5551335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8e7b45400","size":5,"soft_fail":false,"target_va":5375171056,"va":5369619716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33973490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8f2640602","size":5,"soft_fail":false,"target_va":5403593360,"va":5369619865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t33330119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8c793fc01","size":5,"soft_fail":false,"target_va":5402949904,"va":5369619780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7788980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8b4d97600","size":5,"soft_fail":false,"target_va":5377408752,"va":5369619767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7788466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8b2d77600","size":5,"soft_fail":false,"target_va":5377408720,"va":5369620249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7788563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e813d87600","size":5,"soft_fail":false,"target_va":5377408688,"va":5369620120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7788117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e855d67600","size":5,"soft_fail":false,"target_va":5377408224,"va":5369620102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7819492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8e4507700","size":5,"soft_fail":false,"target_va":5377439776,"va":5369620279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7854676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e854da7700","size":5,"soft_fail":false,"target_va":5377475184,"va":5369620503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7848405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e8d5c17700","size":5,"soft_fail":false,"target_va":5377468224,"va":5369619814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369619664,"raw_hex":"e864fdffff","size":5,"soft_fail":false,"target_va":5369619664,"va":5369620327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":29808,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":27200,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":27108,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":27304,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":29164,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":26824,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":27136,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":27556,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":26656,"parent_record_offset":26468,"ancestor_record_offset":26468,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":27944,"parent_record_offset":27920,"ancestor_record_offset":26468,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":72,"record_offset":27920,"parent_record_offset":27864,"ancestor_record_offset":26468,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":27892,"parent_record_offset":27864,"ancestor_record_offset":26468,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":72,"record_offset":27864,"parent_record_offset":27836,"ancestor_record_offset":26468,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":72,"record_offset":30516,"parent_record_offset":30480,"ancestor_record_offset":26468,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":72,"record_offset":30480,"parent_record_offset":30456,"ancestor_record_offset":26468,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":72,"record_offset":26876,"parent_record_offset":26824,"ancestor_record_offset":26468,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":72,"record_offset":27728,"parent_record_offset":27556,"ancestor_record_offset":26468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":72,"record_offset":30548,"parent_record_offset":30516,"ancestor_record_offset":26468,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":72,"record_offset":27160,"parent_record_offset":27136,"ancestor_record_offset":26468,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":72,"record_offset":30576,"parent_record_offset":30516,"ancestor_record_offset":26468,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":72,"record_offset":30608,"parent_record_offset":30480,"ancestor_record_offset":26468,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":26468,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":26776,"parent_record_offset":26752,"ancestor_record_offset":26468,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":72,"record_offset":26752,"parent_record_offset":26728,"ancestor_record_offset":26468,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":27072,"parent_record_offset":26824,"ancestor_record_offset":26468,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":72,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":26468,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":26704,"parent_record_offset":26680,"ancestor_record_offset":26468,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":26680,"parent_record_offset":26656,"ancestor_record_offset":26468,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":27588,"parent_record_offset":27556,"ancestor_record_offset":26468,"inlinee":8485,"source_line":3158,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":72,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26468,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":72,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":26468,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":72,"record_offset":27352,"parent_record_offset":27328,"ancestor_record_offset":26468,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AD7
// 首条位置: {"module":134,"inlinee":10967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":10460,"parent_record_offset":10440,"ancestor_record_offset":10440,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":10508,"parent_record_offset":10484,"ancestor_record_offset":10440,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":10440,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADA
// 首条位置: {"module":134,"inlinee":10970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":11716,"parent_record_offset":11696,"ancestor_record_offset":11696,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":134,"record_offset":11764,"parent_record_offset":11740,"ancestor_record_offset":11696,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":134,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":11696,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00134.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$0
// 信息 ID: 607ff03103a0917f82f21fb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<ref$<std::path::PathBuf>,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<ref$<std::path::PathBuf>,std::path::PathBuf> >; module=134, inline_record=10440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$1
// 信息 ID: a5f2228ca581bf7a1f059fc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$1 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; module=134, inline_record=11696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
// 信息 ID: 6432cf502cd1e2e8dcedebea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["ref$<std::path::PathBuf>", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d2b01 -> 0xdedd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: 17151eb5a521d5daf408934c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf> @ 0xde767 -> 0xde4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf> @ 0xdf057 -> 0xde4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
