// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/save_backup_list.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f697cad8b95bc5f93e7e61507cd85f0a279f0d56e2a5e3280cdc0dd3298550b2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_backup_list.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 490, 570, 576, 1585, 1608, 1644, 1686, 2807
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// IPI IDs: 0x7F74
// 首条位置: {"module":576,"inlinee":32628}
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
// 首条位置: {"module":570,"record_offset":30240,"rva":11436832,"va":5380145952}
// 编译实例 RVAs: 0xae8320
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
// 首条位置: {"module":570,"record_offset":145060,"rva":11469856,"va":5380178976}
// 编译实例 RVAs: 0xaf0420, 0x172c8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2715262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380178976,"raw_hex":"e87e6e2900","size":5,"soft_fail":false,"target_va":5382894288,"va":5380179021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F46
// 首条位置: {"module":570,"inlinee":32582}
// 编译实例 RVAs: 0xae8330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5245538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145968,"raw_hex":"e89ef5afff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380146269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31197874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145968,"raw_hex":"e8b20adc01","size":5,"soft_fail":false,"target_va":5411344000,"va":5380146121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31701005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145968,"raw_hex":"e80db8e301","size":5,"soft_fail":false,"target_va":5411847168,"va":5380146158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2548609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145968,"raw_hex":"e881e32600","size":5,"soft_fail":false,"target_va":5382694656,"va":5380146042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2624550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145968,"raw_hex":"e8260c2800","size":5,"soft_fail":false,"target_va":5382770560,"va":5380146005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":570,"record_offset":30636,"parent_record_offset":30396,"ancestor_record_offset":30396,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":570,"record_offset":30564,"parent_record_offset":30396,"ancestor_record_offset":30396,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":570,"record_offset":30712,"parent_record_offset":30664,"ancestor_record_offset":30396,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":570,"record_offset":30664,"parent_record_offset":30636,"ancestor_record_offset":30396,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":570,"record_offset":30596,"parent_record_offset":30564,"ancestor_record_offset":30396,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A55
// 首条位置: {"module":1585,"inlinee":72277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":26872,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":27088,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1585,"record_offset":25968,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1585,"record_offset":28956,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":8789,"source_line":1366,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1585,"record_offset":26972,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1585,"record_offset":26620,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":51643,"source_line":1651,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":26732,"parent_record_offset":25832,"ancestor_record_offset":25832,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":25832,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":26440,"parent_record_offset":26356,"ancestor_record_offset":25832,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1585,"record_offset":26356,"parent_record_offset":26328,"ancestor_record_offset":25832,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":27284,"parent_record_offset":27256,"ancestor_record_offset":25832,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":27256,"parent_record_offset":27232,"ancestor_record_offset":25832,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":27232,"parent_record_offset":27208,"ancestor_record_offset":25832,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":27508,"parent_record_offset":27472,"ancestor_record_offset":25832,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":25832,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1585,"record_offset":26384,"parent_record_offset":26356,"ancestor_record_offset":25832,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":26996,"parent_record_offset":26972,"ancestor_record_offset":25832,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1585,"record_offset":26408,"parent_record_offset":26384,"ancestor_record_offset":25832,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=131
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":26012,"parent_record_offset":25968,"ancestor_record_offset":25832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":26156,"parent_record_offset":26128,"ancestor_record_offset":25832,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":26204,"parent_record_offset":26180,"ancestor_record_offset":25832,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":25832,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1585,"record_offset":27540,"parent_record_offset":27508,"ancestor_record_offset":25832,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1585,"record_offset":27020,"parent_record_offset":26996,"ancestor_record_offset":25832,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1585,"record_offset":27044,"parent_record_offset":27020,"ancestor_record_offset":25832,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1585,"record_offset":29012,"parent_record_offset":28984,"ancestor_record_offset":25832,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1585,"record_offset":28984,"parent_record_offset":28956,"ancestor_record_offset":25832,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":27568,"parent_record_offset":27508,"ancestor_record_offset":25832,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":27600,"parent_record_offset":27472,"ancestor_record_offset":25832,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1585,"record_offset":26904,"parent_record_offset":26872,"ancestor_record_offset":25832,"inlinee":72279,"source_line":68,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1585,"record_offset":28888,"parent_record_offset":28856,"ancestor_record_offset":25832,"inlinee":72280,"source_line":75,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":26932,"parent_record_offset":26904,"ancestor_record_offset":25832,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":25832,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":26828,"parent_record_offset":26804,"ancestor_record_offset":25832,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":26804,"parent_record_offset":26780,"ancestor_record_offset":25832,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1585,"record_offset":26644,"parent_record_offset":26620,"ancestor_record_offset":25832,"inlinee":51294,"source_line":2069,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1585,"record_offset":26128,"parent_record_offset":26096,"ancestor_record_offset":25832,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1585,"record_offset":26096,"parent_record_offset":26068,"ancestor_record_offset":25832,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":26756,"parent_record_offset":26732,"ancestor_record_offset":25832,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1585,"record_offset":26668,"parent_record_offset":26644,"ancestor_record_offset":25832,"inlinee":51295,"source_line":1129,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1585,"record_offset":26692,"parent_record_offset":26668,"ancestor_record_offset":25832,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A57
// 首条位置: {"module":1585,"inlinee":72279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":26932,"parent_record_offset":26904,"ancestor_record_offset":26904,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A58
// 首条位置: {"module":1585,"inlinee":72280}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":28916,"parent_record_offset":28888,"ancestor_record_offset":28888,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 首条位置: {"module":570,"record_offset":30756,"rva":11437168,"va":5380146288}
// 编译实例 RVAs: 0xae8470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2497702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8a61c2600","size":5,"soft_fail":false,"target_va":5382644528,"va":5380146821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2561413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e885152700","size":5,"soft_fail":false,"target_va":5382708864,"va":5380147446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2619224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e858f72700","size":5,"soft_fail":false,"target_va":5382768048,"va":5380148819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=23 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5309368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e848fcaeff","size":5,"soft_fail":false,"target_va":5374840768,"va":5380150131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5248808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8d8e8afff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380149539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5065740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8f4b3b2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380147239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4630619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8a557b9ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380146646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140884850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2508344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8c8b9d9ff","size":5,"soft_fail":false,"target_va":5377640528,"va":5380148867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2556857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8b9032700","size":5,"soft_fail":false,"target_va":5382704752,"va":5380147890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2681342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8fee92800","size":5,"soft_fail":false,"target_va":5382827728,"va":5380146381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14166df80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12080668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e81c56b800","size":5,"soft_fail":false,"target_va":5392228224,"va":5380147551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31197415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8e708dc01","size":5,"soft_fail":false,"target_va":5411344000,"va":5380146580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31699066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e87ab0e301","size":5,"soft_fail":false,"target_va":5411847152,"va":5380148081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: save_backup_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31699871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e89fb3e301","size":5,"soft_fail":false,"target_va":5411847168,"va":5380147292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2692177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e851142900","size":5,"soft_fail":false,"target_va":5382840208,"va":5380148026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5791682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e83ea0a7ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5380148205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t2535977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e829b22600","size":5,"soft_fail":false,"target_va":5382683584,"va":5380147602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2508186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e89a452600","size":5,"soft_fail":false,"target_va":5382655024,"va":5380146833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31392802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e82204df01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380148969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t384186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8badc0500","size":5,"soft_fail":false,"target_va":5380530560,"va":5380146369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2548174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8cee12600","size":5,"soft_fail":false,"target_va":5382694656,"va":5380146477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2562702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e88e1a2700","size":5,"soft_fail":false,"target_va":5382709584,"va":5380146877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2595943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8679c2700","size":5,"soft_fail":false,"target_va":5382743808,"va":5380147860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2599222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e836a92700","size":5,"soft_fail":false,"target_va":5382745616,"va":5380146389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2624128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380146288,"raw_hex":"e8800a2800","size":5,"soft_fail":false,"target_va":5382770560,"va":5380146427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380146288,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5380147210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":37436,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":30980,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":38000,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":31612,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":570,"record_offset":37520,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":570,"record_offset":33252,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":570,"record_offset":37400,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31820,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":38512,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5731,"source_line":2607,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":37548,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":9143,"source_line":2397,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":570,"record_offset":31288,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":32582,"source_line":54,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":570,"record_offset":31644,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":570,"record_offset":31528,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":570,"record_offset":37372,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":570,"record_offset":31876,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":570,"record_offset":31008,"parent_record_offset":30756,"ancestor_record_offset":30756,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":32876,"parent_record_offset":32852,"ancestor_record_offset":30756,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":32852,"parent_record_offset":32764,"ancestor_record_offset":30756,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":32764,"parent_record_offset":32736,"ancestor_record_offset":30756,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":37484,"parent_record_offset":37460,"ancestor_record_offset":30756,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":38080,"parent_record_offset":38048,"ancestor_record_offset":30756,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":37896,"parent_record_offset":37868,"ancestor_record_offset":30756,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":37868,"parent_record_offset":37844,"ancestor_record_offset":30756,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":30756,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":30756,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":37844,"parent_record_offset":37820,"ancestor_record_offset":30756,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":570,"record_offset":37572,"parent_record_offset":37548,"ancestor_record_offset":30756,"inlinee":32594,"source_line":2501,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":36880,"parent_record_offset":36844,"ancestor_record_offset":30756,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":30756,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":38048,"parent_record_offset":38000,"ancestor_record_offset":30756,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":570,"record_offset":31484,"parent_record_offset":31444,"ancestor_record_offset":30756,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":570,"record_offset":31444,"parent_record_offset":31412,"ancestor_record_offset":30756,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":570,"record_offset":31412,"parent_record_offset":31288,"ancestor_record_offset":30756,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":570,"record_offset":32796,"parent_record_offset":32764,"ancestor_record_offset":30756,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":570,"record_offset":38768,"parent_record_offset":38596,"ancestor_record_offset":30756,"inlinee":9730,"source_line":390,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":37668,"parent_record_offset":37644,"ancestor_record_offset":30756,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":570,"record_offset":31928,"parent_record_offset":31876,"ancestor_record_offset":30756,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$24::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":570,"record_offset":33364,"parent_record_offset":33308,"ancestor_record_offset":30756,"inlinee":32583,"source_line":290,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$59::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":570,"record_offset":33420,"parent_record_offset":33364,"ancestor_record_offset":30756,"inlinee":32584,"source_line":926,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::try_flatten_one::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":570,"record_offset":33660,"parent_record_offset":33604,"ancestor_record_offset":30756,"inlinee":32589,"source_line":871,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$10::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":570,"record_offset":33532,"parent_record_offset":33476,"ancestor_record_offset":30756,"inlinee":32587,"source_line":288,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":570,"record_offset":33476,"parent_record_offset":33420,"ancestor_record_offset":30756,"inlinee":32585,"source_line":83,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":33308,"parent_record_offset":33252,"ancestor_record_offset":30756,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":34776,"parent_record_offset":33660,"ancestor_record_offset":30756,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":33604,"parent_record_offset":33532,"ancestor_record_offset":30756,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":34712,"parent_record_offset":34688,"ancestor_record_offset":30756,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":32820,"parent_record_offset":32796,"ancestor_record_offset":30756,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":38680,"parent_record_offset":38596,"ancestor_record_offset":30756,"inlinee":9727,"source_line":1332,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$37::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":38540,"parent_record_offset":38512,"ancestor_record_offset":30756,"inlinee":32596,"source_line":1385,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$38::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":38596,"parent_record_offset":38568,"ancestor_record_offset":30756,"inlinee":32599,"source_line":1510,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$38::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":38568,"parent_record_offset":38540,"ancestor_record_offset":30756,"inlinee":32598,"source_line":1429,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":38828,"parent_record_offset":38596,"ancestor_record_offset":30756,"inlinee":7218,"source_line":486,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":38712,"parent_record_offset":38596,"ancestor_record_offset":30756,"inlinee":17635,"source_line":870,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":38736,"parent_record_offset":38712,"ancestor_record_offset":30756,"inlinee":17636,"source_line":2579,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":38796,"parent_record_offset":38768,"ancestor_record_offset":30756,"inlinee":4121,"source_line":2021,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":570,"record_offset":37460,"parent_record_offset":37436,"ancestor_record_offset":30756,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":570,"record_offset":34688,"parent_record_offset":34664,"ancestor_record_offset":30756,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":570,"record_offset":31700,"parent_record_offset":31668,"ancestor_record_offset":30756,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":570,"record_offset":31728,"parent_record_offset":31668,"ancestor_record_offset":30756,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31844,"parent_record_offset":31820,"ancestor_record_offset":30756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32564,"parent_record_offset":32536,"ancestor_record_offset":30756,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32612,"parent_record_offset":32588,"ancestor_record_offset":30756,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32588,"parent_record_offset":32564,"ancestor_record_offset":30756,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":36912,"parent_record_offset":36880,"ancestor_record_offset":30756,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":34736,"parent_record_offset":34712,"ancestor_record_offset":30756,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":570,"record_offset":31368,"parent_record_offset":31332,"ancestor_record_offset":30756,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":570,"record_offset":33752,"parent_record_offset":33716,"ancestor_record_offset":30756,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":570,"record_offset":34664,"parent_record_offset":33660,"ancestor_record_offset":30756,"inlinee":32593,"source_line":1908,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":570,"record_offset":33716,"parent_record_offset":33660,"ancestor_record_offset":30756,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":37692,"parent_record_offset":37668,"ancestor_record_offset":30756,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":37716,"parent_record_offset":37692,"ancestor_record_offset":30756,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":37644,"parent_record_offset":37620,"ancestor_record_offset":30756,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":37620,"parent_record_offset":37596,"ancestor_record_offset":30756,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":37596,"parent_record_offset":37572,"ancestor_record_offset":30756,"inlinee":9146,"source_line":1013,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34964,"parent_record_offset":34928,"ancestor_record_offset":30756,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":31552,"parent_record_offset":31528,"ancestor_record_offset":30756,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":31576,"parent_record_offset":31552,"ancestor_record_offset":30756,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34988,"parent_record_offset":34964,"ancestor_record_offset":30756,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":35020,"parent_record_offset":34928,"ancestor_record_offset":30756,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":570,"record_offset":31756,"parent_record_offset":31668,"ancestor_record_offset":30756,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":570,"record_offset":31668,"parent_record_offset":31644,"ancestor_record_offset":30756,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":570,"record_offset":31780,"parent_record_offset":31756,"ancestor_record_offset":30756,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":570,"record_offset":31056,"parent_record_offset":31032,"ancestor_record_offset":30756,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":570,"record_offset":31104,"parent_record_offset":31080,"ancestor_record_offset":30756,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":570,"record_offset":31080,"parent_record_offset":31056,"ancestor_record_offset":30756,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":570,"record_offset":32124,"parent_record_offset":31876,"ancestor_record_offset":30756,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":570,"record_offset":31900,"parent_record_offset":31876,"ancestor_record_offset":30756,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":570,"record_offset":31332,"parent_record_offset":31288,"ancestor_record_offset":30756,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":570,"record_offset":32536,"parent_record_offset":32508,"ancestor_record_offset":30756,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":570,"record_offset":32508,"parent_record_offset":32480,"ancestor_record_offset":30756,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":570,"record_offset":31032,"parent_record_offset":31008,"ancestor_record_offset":30756,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":570,"record_offset":31976,"parent_record_offset":31952,"ancestor_record_offset":30756,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":570,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":30756,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_backup_list::SaveBackupList::into_files
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F77
// 首条位置: {"module":1686,"inlinee":32631}
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
// IPI IDs: 0x6A39
// 首条位置: {"module":490,"inlinee":27193}
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
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile>", "constructor": "core::option::Option", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_backup_list::BackupSaveFile,enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>", "country_core::systems::save::save_backup_list::BackupSaveFile", "enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "core::iter::adapters::flatten::Flatten<core::result::IntoIter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "toolbar::menu::load::ui::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "toolbar::menu::save_error::ui::closure$4::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>", "alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::save::save_backup_list::BackupSaveFile", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::save_error::ui::closure$4::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::save::save_backup_list::BackupSaveFileKind", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_backup_list.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_auto
// 信息 ID: f1c42282e7339dd346e24c23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted
// 信息 ID: 0a4c6cd5754ea691de561c41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$8::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xb4681 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_tod_sequence_entry::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x147a5b -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec1eb -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0xf8f283 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf915a2 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc4313 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::process_tree_cmds::process_tree_cmds @ 0x2267bd8 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x226a1a1 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::ui_place_tree::random_tree_prefab @ 0x226a1c4 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing @ 0x22d3068 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3eb2 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3fb4 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3fd7 -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x22d3cdc -> 0xae8320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_unstuck::sheep_unstuck @ 0x2324dad -> 0xae8320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::backup_metronome_counter
// 信息 ID: 2127cf4865643c736f386c78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=1608, inline_record=30008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::into_files
// 信息 ID: a3765767dba49ccfe43962c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=115396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
// 信息 ID: 725b1b1c59328378a9e4d61a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::save_error::ui::closure$4 @ 0xfc8807 -> 0xae8330 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e22a17 -> 0xae8330 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::save::save_backup_list::SaveBackupList::new_from_session; module=570, inline_record=31288; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=61548; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::SaveBackupList::new_from_session
// 信息 ID: b828d3ca439bc007450bc8a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x146f53c -> 0xae8470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0
// 信息 ID: ece2596c98e8c1eea0ed6b71
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
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1585, inline_record=25832; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1644, inline_record=14936; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2807, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$0
// 信息 ID: 2707ade0c8a4109420148e89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_backup_list::impl$1::iter_backup_files", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1585, inline_record=26904; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1644, inline_record=16004; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2807, inline_record=1568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure$0::closure$1
// 信息 ID: c2a252d22d1d7ee750f7cbb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_backup_list::impl$1::iter_backup_files", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; module=1585, inline_record=28888; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=1644, inline_record=18076; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>; module=2807, inline_record=3644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_backup_list::impl$2::fmt
// 信息 ID: 619f95f9b4e006e2d54d9ed9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
