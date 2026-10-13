// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/fs_util.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e41caa9bd10f549f0020ee2b6c793e792c7ed35f6a2e554d95f1c9131c33e7ae
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fs_util.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 125, 143
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
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":143,"record_offset":32168,"rva":3495936,"va":5372205056}
// 编译实例 RVAs: 0x355800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10503460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e82445a000","size":5,"soft_fail":false,"target_va":5382708864,"va":5372205399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10562029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8ed29a100","size":5,"soft_fail":false,"target_va":5382768048,"va":5372206014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140311690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-279738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e846bbfbff","size":5,"soft_fail":false,"target_va":5371926160,"va":5372205893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403117f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-278767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e811bffbff","size":5,"soft_fail":false,"target_va":5371926512,"va":5372205274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2689352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e848092900","size":5,"soft_fail":false,"target_va":5374896624,"va":5372207267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2875815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8a7e12b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372205684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10622622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e89e16a200","size":5,"soft_fail":false,"target_va":5382827728,"va":5372205101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39118734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e88ee75402","size":5,"soft_fail":false,"target_va":5411323904,"va":5372205165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39641912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e838e35c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372205251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t10477592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e818e09f00","size":5,"soft_fail":false,"target_va":5382683584,"va":5372205987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10504427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8eb48a000","size":5,"soft_fail":false,"target_va":5382709584,"va":5372205152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10503923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8f346a000","size":5,"soft_fail":false,"target_va":5382709552,"va":5372205624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10504020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e85447a000","size":5,"soft_fail":false,"target_va":5382709520,"va":5372205495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10503526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e86645a000","size":5,"soft_fail":false,"target_va":5382709008,"va":5372205477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10538421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8b5cda000","size":5,"soft_fail":false,"target_va":5382744080,"va":5372205654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10573108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e83455a100","size":5,"soft_fail":false,"target_va":5382778992,"va":5372205879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10565356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e8ec36a100","size":5,"soft_fail":false,"target_va":5382770560,"va":5372205199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372205056,"raw_hex":"e865f4ffff","size":5,"soft_fail":false,"target_va":5372202736,"va":5372205702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":143,"record_offset":32368,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":143,"record_offset":35596,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":32940,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":32848,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":33044,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":34952,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":32564,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":32876,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":33296,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":33128,"parent_record_offset":32168,"ancestor_record_offset":32168,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":143,"record_offset":33692,"parent_record_offset":33668,"ancestor_record_offset":32168,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":143,"record_offset":33668,"parent_record_offset":33612,"ancestor_record_offset":32168,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":33640,"parent_record_offset":33612,"ancestor_record_offset":32168,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":33612,"parent_record_offset":33588,"ancestor_record_offset":32168,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":33588,"parent_record_offset":33564,"ancestor_record_offset":32168,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":143,"record_offset":36328,"parent_record_offset":36292,"ancestor_record_offset":32168,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":143,"record_offset":36292,"parent_record_offset":36268,"ancestor_record_offset":32168,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":143,"record_offset":32616,"parent_record_offset":32564,"ancestor_record_offset":32168,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":143,"record_offset":33468,"parent_record_offset":33296,"ancestor_record_offset":32168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":143,"record_offset":36360,"parent_record_offset":36328,"ancestor_record_offset":32168,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":32900,"parent_record_offset":32876,"ancestor_record_offset":32168,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":143,"record_offset":36388,"parent_record_offset":36328,"ancestor_record_offset":32168,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":143,"record_offset":36420,"parent_record_offset":36292,"ancestor_record_offset":32168,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":32168,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":32512,"parent_record_offset":32488,"ancestor_record_offset":32168,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":32488,"parent_record_offset":32464,"ancestor_record_offset":32168,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":32812,"parent_record_offset":32564,"ancestor_record_offset":32168,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":32588,"parent_record_offset":32564,"ancestor_record_offset":32168,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":32440,"parent_record_offset":32416,"ancestor_record_offset":32168,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":32416,"parent_record_offset":32392,"ancestor_record_offset":32168,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$80::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":33328,"parent_record_offset":33296,"ancestor_record_offset":32168,"inlinee":5140,"source_line":3338,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":32392,"parent_record_offset":32368,"ancestor_record_offset":32168,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":143,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":32168,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":143,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":32168,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/00143.jsonl.gz"}
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
// 首条位置: {"module":143,"record_offset":22272,"rva":3493616,"va":5372202736}
// 编译实例 RVAs: 0x354ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10505780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8344ea000","size":5,"soft_fail":false,"target_va":5382708864,"va":5372203079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10564349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8fd32a100","size":5,"soft_fail":false,"target_va":5382768048,"va":5372203694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140311690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-277418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e856c4fbff","size":5,"soft_fail":false,"target_va":5371926160,"va":5372203573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403117f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-276454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e81ac8fbff","size":5,"soft_fail":false,"target_va":5371926512,"va":5372202961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2691592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e808122900","size":5,"soft_fail":false,"target_va":5374896624,"va":5372205027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2878135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8b7ea2b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372203364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10624935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8a71fa200","size":5,"soft_fail":false,"target_va":5382827728,"va":5372202788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39121047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e897f05402","size":5,"soft_fail":false,"target_va":5411323904,"va":5372202852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: fs_util.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39644225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e841ec5c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372202938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t10479912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e828e99f00","size":5,"soft_fail":false,"target_va":5382683584,"va":5372203667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10506740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8f451a000","size":5,"soft_fail":false,"target_va":5382709584,"va":5372202839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10506243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e80350a000","size":5,"soft_fail":false,"target_va":5382709552,"va":5372203304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10506340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e86450a000","size":5,"soft_fail":false,"target_va":5382709520,"va":5372203175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10505846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8764ea000","size":5,"soft_fail":false,"target_va":5382709008,"va":5372203157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10540741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8c5d6a000","size":5,"soft_fail":false,"target_va":5382744080,"va":5372203334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10575428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8445ea100","size":5,"soft_fail":false,"target_va":5382778992,"va":5372203559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10567669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e8f53fa100","size":5,"soft_fail":false,"target_va":5382770560,"va":5372202886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372202736,"raw_hex":"e875fdffff","size":5,"soft_fail":false,"target_va":5372202736,"va":5372203382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":143,"record_offset":25660,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":23004,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":22912,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":23108,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":25016,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":22628,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":22940,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":23360,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":22460,"parent_record_offset":22272,"ancestor_record_offset":22272,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":143,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":22272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":143,"record_offset":23732,"parent_record_offset":23676,"ancestor_record_offset":22272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":23704,"parent_record_offset":23676,"ancestor_record_offset":22272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":22272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":143,"record_offset":23652,"parent_record_offset":23628,"ancestor_record_offset":22272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":143,"record_offset":26392,"parent_record_offset":26356,"ancestor_record_offset":22272,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":143,"record_offset":26356,"parent_record_offset":26332,"ancestor_record_offset":22272,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":143,"record_offset":22680,"parent_record_offset":22628,"ancestor_record_offset":22272,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=116
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":143,"record_offset":23532,"parent_record_offset":23360,"ancestor_record_offset":22272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":143,"record_offset":26424,"parent_record_offset":26392,"ancestor_record_offset":22272,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":143,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":22272,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":143,"record_offset":26452,"parent_record_offset":26392,"ancestor_record_offset":22272,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":143,"record_offset":26484,"parent_record_offset":26356,"ancestor_record_offset":22272,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":22272,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":22580,"parent_record_offset":22556,"ancestor_record_offset":22272,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":143,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":22272,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":22876,"parent_record_offset":22628,"ancestor_record_offset":22272,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":143,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":22272,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":22272,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":22272,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$80::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":23392,"parent_record_offset":23360,"ancestor_record_offset":22272,"inlinee":5140,"source_line":3338,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":143,"record_offset":22704,"parent_record_offset":22680,"ancestor_record_offset":22272,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":143,"record_offset":23132,"parent_record_offset":23108,"ancestor_record_offset":22272,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":143,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":22272,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/00143.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3091
// 首条位置: {"module":125,"inlinee":12433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":17820,"parent_record_offset":17800,"ancestor_record_offset":17800,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":17800,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":125,"record_offset":17844,"parent_record_offset":17820,"ancestor_record_offset":17800,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::fs_util::copy_folder::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3090
// 首条位置: {"module":125,"inlinee":12432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":17132,"parent_record_offset":17112,"ancestor_record_offset":17112,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":17112,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":125,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":17112,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00125.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$0
// 信息 ID: 9c3816ffa2afe91ae56b1d05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<std::path::PathBuf,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$0<std::path::PathBuf,std::path::PathBuf> >; module=125, inline_record=17800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder::closure$1
// 信息 ID: 393d6dbbabd1244a977bd804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::fs_util::copy_folder", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::fs_util::copy_folder::closure_env$1 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env$1<std::path::PathBuf,std::path::PathBuf> >; module=125, inline_record=17112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
// 信息 ID: 307c79c85bf9c9d6a4c5b005
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["std::path::PathBuf"]; 字段布局 = unknown。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["ref$<std::path::PathBuf>", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a36d6 -> 0x355800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: c0baf1692ff290ddd8c5e0ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::fs_util::copy_folder -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::fs_util::copy_folder<std::path::PathBuf,std::path::PathBuf> @ 0x355176 -> 0x354ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf> @ 0x355a86 -> 0x354ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
