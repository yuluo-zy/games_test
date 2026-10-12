// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/aftermath.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 04dba6f40ae8b75762a5cf0db7515efb0c0b0988cbe64300cf2c4ecf36450313
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: aftermath.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 79
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use aftermath_rs; // namespace candidate
//   package identities: [{"name":"aftermath-rs","source":"local_or_path","version":"0.1.3"}]
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::gpu_crash_dump_zip_path
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2987
// 首条位置: {"module":79,"inlinee":10631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":31016,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":79,"record_offset":30864,"rva":2376800,"va":5371085920}
// 编译实例 RVAs: 0x244460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11558285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e88d5db000","size":5,"soft_fail":false,"target_va":5382644528,"va":5371086238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3814020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e884323a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5371086711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3995168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e820f63c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371086331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4429923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e863984300","size":5,"soft_fail":false,"target_va":5375516032,"va":5371086104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11741729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e8212ab300","size":5,"soft_fail":false,"target_va":5382827728,"va":5371085994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9803579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e83b979500","size":5,"soft_fail":false,"target_va":5380889536,"va":5371085952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11571495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e82791b000","size":5,"soft_fail":false,"target_va":5382657856,"va":5371086356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11658089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e869e3b100","size":5,"soft_fail":false,"target_va":5382744080,"va":5371085986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11659321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e839e8b100","size":5,"soft_fail":false,"target_va":5382745328,"va":5371086002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11686081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e8c150b200","size":5,"soft_fail":false,"target_va":5382772112,"va":5371086026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11667401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085920,"raw_hex":"e8c907b200","size":5,"soft_fail":false,"target_va":5382753792,"va":5371086386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371085920,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371086302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":79,"record_offset":31520,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":31728,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":32528,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":31552,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":31436,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":79,"record_offset":31212,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31072,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::aftermath::gpu_crash_dump_zip_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/aftermath.rs
//     evidence={"module":79,"record_offset":31016,"parent_record_offset":30864,"ancestor_record_offset":30864,"inlinee":10631,"source_line":16,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":32152,"parent_record_offset":32128,"ancestor_record_offset":30864,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":32128,"parent_record_offset":32040,"ancestor_record_offset":30864,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":79,"record_offset":32040,"parent_record_offset":32012,"ancestor_record_offset":30864,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":33480,"parent_record_offset":33452,"ancestor_record_offset":30864,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":33452,"parent_record_offset":33428,"ancestor_record_offset":30864,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":33428,"parent_record_offset":33404,"ancestor_record_offset":30864,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":79,"record_offset":32072,"parent_record_offset":32040,"ancestor_record_offset":30864,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":79,"record_offset":33080,"parent_record_offset":33048,"ancestor_record_offset":30864,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":79,"record_offset":33140,"parent_record_offset":33108,"ancestor_record_offset":30864,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":79,"record_offset":31236,"parent_record_offset":31212,"ancestor_record_offset":30864,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":79,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":30864,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":79,"record_offset":32096,"parent_record_offset":32072,"ancestor_record_offset":30864,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":79,"record_offset":33108,"parent_record_offset":33048,"ancestor_record_offset":30864,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":79,"record_offset":32684,"parent_record_offset":32648,"ancestor_record_offset":30864,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":79,"record_offset":32800,"parent_record_offset":32772,"ancestor_record_offset":30864,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":31608,"parent_record_offset":31576,"ancestor_record_offset":30864,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":31636,"parent_record_offset":31576,"ancestor_record_offset":30864,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":31756,"parent_record_offset":31728,"ancestor_record_offset":30864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":32888,"parent_record_offset":32772,"ancestor_record_offset":30864,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":31840,"parent_record_offset":31812,"ancestor_record_offset":30864,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":30864,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":30864,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":32964,"parent_record_offset":32936,"ancestor_record_offset":30864,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":32912,"parent_record_offset":32888,"ancestor_record_offset":30864,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":32992,"parent_record_offset":32936,"ancestor_record_offset":30864,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":31460,"parent_record_offset":31436,"ancestor_record_offset":30864,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":31484,"parent_record_offset":31460,"ancestor_record_offset":30864,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":33252,"parent_record_offset":32528,"ancestor_record_offset":30864,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":31664,"parent_record_offset":31576,"ancestor_record_offset":30864,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":31576,"parent_record_offset":31552,"ancestor_record_offset":30864,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":31688,"parent_record_offset":31664,"ancestor_record_offset":30864,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":30864,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":79,"record_offset":31120,"parent_record_offset":31096,"ancestor_record_offset":30864,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":79,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":30864,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":79,"record_offset":31144,"parent_record_offset":31120,"ancestor_record_offset":30864,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":79,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":30864,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":79,"record_offset":32848,"parent_record_offset":32824,"ancestor_record_offset":30864,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":79,"record_offset":32712,"parent_record_offset":32684,"ancestor_record_offset":30864,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":79,"record_offset":33048,"parent_record_offset":32648,"ancestor_record_offset":30864,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":79,"record_offset":32576,"parent_record_offset":32552,"ancestor_record_offset":30864,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":79,"record_offset":31812,"parent_record_offset":31784,"ancestor_record_offset":30864,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":79,"record_offset":31784,"parent_record_offset":31756,"ancestor_record_offset":30864,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":30864,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31096,"parent_record_offset":31072,"ancestor_record_offset":30864,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31284,"parent_record_offset":31260,"ancestor_record_offset":30864,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":31260,"parent_record_offset":31236,"ancestor_record_offset":30864,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":79,"record_offset":32740,"parent_record_offset":32712,"ancestor_record_offset":30864,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":79,"record_offset":32772,"parent_record_offset":32740,"ancestor_record_offset":30864,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":79,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":30864,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":79,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":30864,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::impl$1::dumped
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":79,"record_offset":33912,"rva":2377616,"va":5371086736}
// 编译实例 RVAs: 0x244790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11557588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8d45ab000","size":5,"soft_fail":false,"target_va":5382644528,"va":5371086935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f57f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1373159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8190cebff","size":5,"soft_fail":false,"target_va":5369714672,"va":5371087826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f6600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1369690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8a619ebff","size":5,"soft_fail":false,"target_va":5369718272,"va":5371087957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f6e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1367346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8ce22ebff","size":5,"soft_fail":false,"target_va":5369720448,"va":5371087789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f7dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1364359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8792eebff","size":5,"soft_fail":false,"target_va":5369724352,"va":5371088706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f8280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1363602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e86e31ebff","size":5,"soft_fail":false,"target_va":5369725568,"va":5371089165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-931176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e898caf1ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371089299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403108d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t833796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e804b90c00","size":5,"soft_fail":false,"target_va":5371922640,"va":5371088839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2540438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e896c32600","size":5,"soft_fail":false,"target_va":5373629488,"va":5371089045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d90f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3752626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8b2423900","size":5,"soft_fail":false,"target_va":5374841072,"va":5371088441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3811359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e81f283a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5371089372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3887585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8e1513b00","size":5,"soft_fail":false,"target_va":5374976736,"va":5371089146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3993627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e81bf03c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371087872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4429215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e89f954300","size":5,"soft_fail":false,"target_va":5375516032,"va":5371086812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc05d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9944299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8ebbc9700","size":5,"soft_fail":false,"target_va":5381031376,"va":5371087072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40233001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e829e86502","size":5,"soft_fail":false,"target_va":5411321136,"va":5371088130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40235040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e820f06502","size":5,"soft_fail":false,"target_va":5411323904,"va":5371088859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4180
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40236113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e851f46502","size":5,"soft_fail":false,"target_va":5411324288,"va":5371088170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11752495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e82f54b300","size":5,"soft_fail":false,"target_va":5382840208,"va":5371087708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11566349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e80d7db000","size":5,"soft_fail":false,"target_va":5382655024,"va":5371088670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40403299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e863816802","size":5,"soft_fail":false,"target_va":5411492368,"va":5371089064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40406409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8898d6802","size":5,"soft_fail":false,"target_va":5411494800,"va":5371088386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10736278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e896d2a300","size":5,"soft_fail":false,"target_va":5381823600,"va":5371087317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10736119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e8f7d1a300","size":5,"soft_fail":false,"target_va":5381824016,"va":5371087892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9800781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e84d8c9500","size":5,"soft_fail":false,"target_va":5380889536,"va":5371088750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11606623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e85f1ab100","size":5,"soft_fail":false,"target_va":5382694656,"va":5371088028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11655288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371086736,"raw_hex":"e878d8b100","size":5,"soft_fail":false,"target_va":5382744080,"va":5371088787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":79,"record_offset":34140,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":79,"record_offset":38420,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":38276,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":34172,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":34056,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::aftermath::NvAftermathDelegate::try_write_dump
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/aftermath.rs
//     evidence={"module":79,"record_offset":34340,"parent_record_offset":33912,"ancestor_record_offset":33912,"inlinee":10636,"source_line":61,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":36356,"parent_record_offset":36332,"ancestor_record_offset":33912,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":36332,"parent_record_offset":36276,"ancestor_record_offset":33912,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":79,"record_offset":35916,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":79,"record_offset":35964,"parent_record_offset":35940,"ancestor_record_offset":33912,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":33912,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36276,"parent_record_offset":36252,"ancestor_record_offset":33912,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36252,"parent_record_offset":36228,"ancestor_record_offset":33912,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":79,"record_offset":34492,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":79,"record_offset":36140,"parent_record_offset":36100,"ancestor_record_offset":33912,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":79,"record_offset":36100,"parent_record_offset":36064,"ancestor_record_offset":33912,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":79,"record_offset":36064,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":79,"record_offset":35020,"parent_record_offset":34992,"ancestor_record_offset":33912,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":79,"record_offset":35080,"parent_record_offset":35048,"ancestor_record_offset":33912,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":79,"record_offset":36000,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":79,"record_offset":35440,"parent_record_offset":35408,"ancestor_record_offset":33912,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":79,"record_offset":35408,"parent_record_offset":35376,"ancestor_record_offset":33912,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":79,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":33912,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":34224,"parent_record_offset":34196,"ancestor_record_offset":33912,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":34252,"parent_record_offset":34196,"ancestor_record_offset":33912,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":79,"record_offset":35116,"parent_record_offset":34964,"ancestor_record_offset":33912,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":36180,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":35700,"parent_record_offset":35676,"ancestor_record_offset":33912,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":79,"record_offset":36028,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":79,"record_offset":36744,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":36528,"parent_record_offset":36464,"ancestor_record_offset":33912,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34716,"parent_record_offset":34680,"ancestor_record_offset":33912,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":33912,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":36552,"parent_record_offset":36528,"ancestor_record_offset":33912,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":33912,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34104,"parent_record_offset":34080,"ancestor_record_offset":33912,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35336,"parent_record_offset":35296,"ancestor_record_offset":33912,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35376,"parent_record_offset":35336,"ancestor_record_offset":33912,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35484,"parent_record_offset":35296,"ancestor_record_offset":33912,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35296,"parent_record_offset":35248,"ancestor_record_offset":33912,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":34992,"parent_record_offset":34964,"ancestor_record_offset":33912,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":35048,"parent_record_offset":34992,"ancestor_record_offset":33912,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":35576,"parent_record_offset":35248,"ancestor_record_offset":33912,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":79,"record_offset":34856,"parent_record_offset":34832,"ancestor_record_offset":33912,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":79,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":33912,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35848,"parent_record_offset":35824,"ancestor_record_offset":33912,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35824,"parent_record_offset":35192,"ancestor_record_offset":33912,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35676,"parent_record_offset":35248,"ancestor_record_offset":33912,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34964,"parent_record_offset":34904,"ancestor_record_offset":33912,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35248,"parent_record_offset":35220,"ancestor_record_offset":33912,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34880,"parent_record_offset":34856,"ancestor_record_offset":33912,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34936,"parent_record_offset":34904,"ancestor_record_offset":33912,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":33912,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35220,"parent_record_offset":35192,"ancestor_record_offset":33912,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":34652,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":36436,"parent_record_offset":36408,"ancestor_record_offset":33912,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":34280,"parent_record_offset":34196,"ancestor_record_offset":33912,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":33912,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":34304,"parent_record_offset":34280,"ancestor_record_offset":33912,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":34680,"parent_record_offset":34652,"ancestor_record_offset":33912,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":36464,"parent_record_offset":36436,"ancestor_record_offset":33912,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":34772,"parent_record_offset":34680,"ancestor_record_offset":33912,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":36500,"parent_record_offset":36464,"ancestor_record_offset":33912,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":33912,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":34808,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":35168,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":33912,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::aftermath::gpu_crash_dump_zip_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/aftermath.rs
//     evidence={"module":79,"record_offset":37440,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":10631,"source_line":16,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] zip::write::FileOptions::unix_permissions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34624,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":10638,"source_line":178,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] zip::write::impl$9::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34592,"parent_record_offset":34520,"ancestor_record_offset":33912,"inlinee":8658,"source_line":108,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] zip::write::zip_writer::ZipWriter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34520,"parent_record_offset":34340,"ancestor_record_offset":33912,"inlinee":8688,"source_line":327,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::impl$1::shader_debug_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":79,"record_offset":38480,"rva":2380608,"va":5371089728}
// 编译实例 RVAs: 0x245340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11554567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8074fb000","size":5,"soft_fail":false,"target_va":5382644528,"va":5371089956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] aftermath_rs::get_shader_debug_info_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"aftermath-rs","version":"0.1.3","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10269353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8a9b29c00","size":5,"soft_fail":false,"target_va":5381359136,"va":5371089778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-932145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8cfc6f1ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371090268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t541859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8a3440800","size":5,"soft_fail":false,"target_va":5371631968,"va":5371090104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3991445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e895e73c00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371090038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3991366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e846e73c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371090133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4426202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8da894300","size":5,"soft_fail":false,"target_va":5375516032,"va":5371089825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: aftermath.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40757092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e864e76d02","size":5,"soft_fail":false,"target_va":5411847168,"va":5371090071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40450735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8af3a6902","size":5,"soft_fail":false,"target_va":5411540963,"va":5371090223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40402178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e8027d6802","size":5,"soft_fail":false,"target_va":5411492368,"va":5371090185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40404595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e873866802","size":5,"soft_fail":false,"target_va":5411494800,"va":5371090200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10733598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e81ec8a300","size":5,"soft_fail":false,"target_va":5381823600,"va":5371089997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10733866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371089728,"raw_hex":"e82ac9a300","size":5,"soft_fail":false,"target_va":5381824016,"va":5371090145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":79,"record_offset":39080,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":79,"record_offset":38724,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":39752,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":38924,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":39668,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":38756,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":38640,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":39724,"parent_record_offset":38480,"ancestor_record_offset":38480,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":38480,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":38480,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":38480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":39492,"parent_record_offset":39316,"ancestor_record_offset":38480,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":39904,"parent_record_offset":39848,"ancestor_record_offset":38480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39256,"parent_record_offset":39228,"ancestor_record_offset":38480,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39876,"parent_record_offset":39848,"ancestor_record_offset":38480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39848,"parent_record_offset":39824,"ancestor_record_offset":38480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39316,"parent_record_offset":39284,"ancestor_record_offset":38480,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39284,"parent_record_offset":39256,"ancestor_record_offset":38480,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39824,"parent_record_offset":39800,"ancestor_record_offset":38480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":39352,"parent_record_offset":39316,"ancestor_record_offset":38480,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":79,"record_offset":39192,"parent_record_offset":39164,"ancestor_record_offset":38480,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":79,"record_offset":39164,"parent_record_offset":39136,"ancestor_record_offset":38480,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":79,"record_offset":39108,"parent_record_offset":39080,"ancestor_record_offset":38480,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":79,"record_offset":39136,"parent_record_offset":39108,"ancestor_record_offset":38480,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":79,"record_offset":39228,"parent_record_offset":39192,"ancestor_record_offset":38480,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":79,"record_offset":39376,"parent_record_offset":39352,"ancestor_record_offset":38480,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":79,"record_offset":39400,"parent_record_offset":39376,"ancestor_record_offset":38480,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":79,"record_offset":39692,"parent_record_offset":39668,"ancestor_record_offset":38480,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":79,"record_offset":39616,"parent_record_offset":39592,"ancestor_record_offset":38480,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":79,"record_offset":39448,"parent_record_offset":39424,"ancestor_record_offset":38480,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":79,"record_offset":39424,"parent_record_offset":39400,"ancestor_record_offset":38480,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":38808,"parent_record_offset":38780,"ancestor_record_offset":38480,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":79,"record_offset":38836,"parent_record_offset":38780,"ancestor_record_offset":38480,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":79,"record_offset":39592,"parent_record_offset":39192,"ancestor_record_offset":38480,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":39776,"parent_record_offset":39752,"ancestor_record_offset":38480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":40100,"parent_record_offset":40036,"ancestor_record_offset":38480,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":38988,"parent_record_offset":38952,"ancestor_record_offset":38480,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":38664,"parent_record_offset":38640,"ancestor_record_offset":38480,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":40124,"parent_record_offset":40100,"ancestor_record_offset":38480,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":39012,"parent_record_offset":38988,"ancestor_record_offset":38480,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":38688,"parent_record_offset":38664,"ancestor_record_offset":38480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":40008,"parent_record_offset":39980,"ancestor_record_offset":38480,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":38864,"parent_record_offset":38780,"ancestor_record_offset":38480,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":79,"record_offset":38780,"parent_record_offset":38756,"ancestor_record_offset":38480,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":79,"record_offset":38888,"parent_record_offset":38864,"ancestor_record_offset":38480,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":38952,"parent_record_offset":38924,"ancestor_record_offset":38480,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":40036,"parent_record_offset":40008,"ancestor_record_offset":38480,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":39044,"parent_record_offset":38952,"ancestor_record_offset":38480,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":40072,"parent_record_offset":40036,"ancestor_record_offset":38480,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::impl$1::description
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":79,"record_offset":40172,"rva":244608,"va":5368953728}
// 编译实例 RVAs: 0x3bb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::aftermath::NvAftermathDelegate::try_write_dump
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298C
// 首条位置: {"module":79,"inlinee":10636}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":79,"record_offset":35916,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":79,"record_offset":34492,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":79,"record_offset":36064,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":79,"record_offset":36000,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":36180,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":79,"record_offset":36028,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":79,"record_offset":36744,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":34652,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":34808,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":35168,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::aftermath::gpu_crash_dump_zip_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/aftermath.rs
//     evidence={"module":79,"record_offset":37440,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":10631,"source_line":16,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] zip::write::FileOptions::unix_permissions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34624,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":10638,"source_line":178,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_direct] zip::write::zip_writer::ZipWriter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34520,"parent_record_offset":34340,"ancestor_record_offset":34340,"inlinee":8688,"source_line":327,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":36356,"parent_record_offset":36332,"ancestor_record_offset":34340,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":79,"record_offset":36332,"parent_record_offset":36276,"ancestor_record_offset":34340,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":79,"record_offset":35964,"parent_record_offset":35940,"ancestor_record_offset":34340,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36304,"parent_record_offset":36276,"ancestor_record_offset":34340,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36276,"parent_record_offset":36252,"ancestor_record_offset":34340,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":79,"record_offset":36252,"parent_record_offset":36228,"ancestor_record_offset":34340,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":79,"record_offset":34564,"parent_record_offset":34520,"ancestor_record_offset":34340,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":79,"record_offset":36140,"parent_record_offset":36100,"ancestor_record_offset":34340,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":79,"record_offset":36100,"parent_record_offset":36064,"ancestor_record_offset":34340,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":79,"record_offset":35020,"parent_record_offset":34992,"ancestor_record_offset":34340,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":79,"record_offset":35080,"parent_record_offset":35048,"ancestor_record_offset":34340,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":79,"record_offset":35440,"parent_record_offset":35408,"ancestor_record_offset":34340,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":79,"record_offset":35408,"parent_record_offset":35376,"ancestor_record_offset":34340,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":79,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":34340,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":79,"record_offset":35116,"parent_record_offset":34964,"ancestor_record_offset":34340,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":79,"record_offset":36204,"parent_record_offset":36180,"ancestor_record_offset":34340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":79,"record_offset":35700,"parent_record_offset":35676,"ancestor_record_offset":34340,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":36528,"parent_record_offset":36464,"ancestor_record_offset":34340,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34716,"parent_record_offset":34680,"ancestor_record_offset":34340,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":36552,"parent_record_offset":36528,"ancestor_record_offset":34340,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":79,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":34340,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35336,"parent_record_offset":35296,"ancestor_record_offset":34340,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35376,"parent_record_offset":35336,"ancestor_record_offset":34340,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35484,"parent_record_offset":35296,"ancestor_record_offset":34340,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":79,"record_offset":35296,"parent_record_offset":35248,"ancestor_record_offset":34340,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":34992,"parent_record_offset":34964,"ancestor_record_offset":34340,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":35048,"parent_record_offset":34992,"ancestor_record_offset":34340,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":79,"record_offset":35576,"parent_record_offset":35248,"ancestor_record_offset":34340,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":79,"record_offset":34856,"parent_record_offset":34832,"ancestor_record_offset":34340,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":79,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":34340,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35848,"parent_record_offset":35824,"ancestor_record_offset":34340,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35824,"parent_record_offset":35192,"ancestor_record_offset":34340,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35676,"parent_record_offset":35248,"ancestor_record_offset":34340,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34964,"parent_record_offset":34904,"ancestor_record_offset":34340,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35248,"parent_record_offset":35220,"ancestor_record_offset":34340,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34880,"parent_record_offset":34856,"ancestor_record_offset":34340,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34936,"parent_record_offset":34904,"ancestor_record_offset":34340,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34340,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":79,"record_offset":35220,"parent_record_offset":35192,"ancestor_record_offset":34340,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":79,"record_offset":36436,"parent_record_offset":36408,"ancestor_record_offset":34340,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":34680,"parent_record_offset":34652,"ancestor_record_offset":34340,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":79,"record_offset":36464,"parent_record_offset":36436,"ancestor_record_offset":34340,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":34772,"parent_record_offset":34680,"ancestor_record_offset":34340,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":79,"record_offset":36500,"parent_record_offset":36464,"ancestor_record_offset":34340,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":79,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":34340,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":79,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":34340,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] zip::write::impl$9::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/write.rs
//     evidence={"module":79,"record_offset":34592,"parent_record_offset":34520,"ancestor_record_offset":34340,"inlinee":8658,"source_line":108,"archive":"_pdb/B/modules/00079.jsonl.gz"}
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
// 完整结构及原始类型记录: aftermath.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::aftermath::NvAftermathDelegate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "aftermath_rs::Aftermath::new<tiny_glade::aftermath::NvAftermathDelegate>", "constructor": "aftermath_rs::Aftermath::new", "arguments": ["tiny_glade::aftermath::NvAftermathDelegate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::aftermath::NvAftermathDelegate>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::aftermath::NvAftermathDelegate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: aftermath.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "tiny_glade::aftermath::NvAftermathDelegate", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: aftermath.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 154; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "country_core::systems::ui::resources::slider3d::Slider3d", "bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >", "tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::system::commands::Commands", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "country_core::systems::ui::resources::slider3d::Slider3d", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::EditRoofCmd>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<system_decorator::ui_systems::ui_place_decorator::EvInitPlaceDecoratorInteractionState> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,system_decorator::ui_systems::ui_move_decorator::RaycastResources,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsPlaced> >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,tuple$<bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_place_decorator::PlayedConnectSoundLastFrame>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd> >)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<system_decorator::ui_systems::ui_place_decorator::EvInitPlaceDecoratorInteractionState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,system_decorator::ui_systems::ui_move_decorator::RaycastResources,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsPlaced> >", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "tuple$<bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_place_decorator::PlayedConnectSoundLastFrame>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_decorator::ui_systems::ui_place_decorator::EvInitPlaceDecoratorInteractionState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemStairsPlaced>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::ui_systems::ui_place_decorator::PlayedConnectSoundLastFrame>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tiny_glade::rollercoaster::RollercoasterState> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,tuple$<country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> > >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::critters::sheep::SheepAudio>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> >,tuple$<> >,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::SheepAccessory>,bevy_ecs::query::filter::With<bevy_ecs::entity_disabling::Disabled> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tiny_glade::rollercoaster::RollercoasterState> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "tuple$<country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >", "bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> > >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::critters::sheep::SheepAudio>,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> >,tuple$<> >", "bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::SheepAccessory>,bevy_ecs::query::filter::With<bevy_ecs::entity_disabling::Disabled> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tiny_glade::rollercoaster::RollercoasterState> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::query::filter::Without<country_core::resources::critters::sheep::Sheep> > >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::critters::sheep::SheepAudio>,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> >,tuple$<> >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::SheepAccessory>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<bevy_ecs::entity_disabling::Disabled>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::SheepAccessory>,bevy_ecs::query::filter::With<bevy_ecs::entity_disabling::Disabled> > >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::NvAftermathDelegate::try_write_dump
// 信息 ID: 33c3b64abff488c1e370381a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::aftermath::impl$1::dumped; module=79, inline_record=34340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::gpu_crash_dump_zip_path
// 信息 ID: 4ec3cb696a9d4341ae155398
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::aftermath::impl$0::default; module=79, inline_record=31016; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::aftermath::impl$1::dumped; module=79, inline_record=37440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::impl$0::default
// 信息 ID: f6658ca0b4954130a8c3a59d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x593ddb -> 0x244460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::impl$1::description
// 信息 ID: 010384849801787a89538fa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0094895827cf75842dfd9081: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 01032eead5fe87cdf7af3db3: void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0230a00f6501f069c8d8be52: void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// [泛型宿主/闭包/直接引用方参数候选] 06f136cf5a2d63be4470c1a9: void (*)(bevy_ecs::event::reader::EventReader<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 077651db72eb55a761f661a4: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::AutoClutter>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,enum2$<core::option::Option<ref$<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 08b4757565a23d44078797c4: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0aa172b68cab4584f80e4193: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 0aea34a098357f624f5c6f72: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 其余候选接口见完整附档，共 154 条。
// 引用方: 原程序直接转移 646 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0x23cc -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0x242c -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x3365d -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x33672 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x336bd -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests> >,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,country_core::systems::ui::resources::slider3d::Slider3d,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)> > @ 0x336d2 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x404eb -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x4055b -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0x47e28 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0x47e98 -> 0x3bb80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > > @ 0x4934a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global> @ 0x4cf86 -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global> @ 0x4ef7a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,alloc::alloc::Global> @ 0x4f75b -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global> @ 0x51b8a -> 0x3bb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > @ 0xaf24c -> 0x3bb80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::impl$1::dumped
// 信息 ID: 371fa08ab7afa4ecb15d9558
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::aftermath::impl$1::shader_debug_info
// 信息 ID: 2df7cb73c3f3bb0e652083cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
