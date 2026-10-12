// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/panic_dump.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c09ab63e3088edbeb2eb21d1d050d9644921a545b3631b18fa6b7ba6f9a4dd27
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_dump.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 157
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":14892,"rva":3853424,"va":5372562544}
// 编译实例 RVAs: 0x3acc70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2337757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8ddab2300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372562974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2518798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e80e6f2600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372562701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10265084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8fca19c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372562639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38758344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8c8674f02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372562787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8326932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8140f7f00","size":5,"soft_fail":false,"target_va":5380889536,"va":5372562599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10131928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8d8999a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372562723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10181481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e8695b9b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372562594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10216316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372562544,"raw_hex":"e87ce39b00","size":5,"soft_fail":false,"target_va":5382778992,"va":5372562671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":16160,"parent_record_offset":14892,"ancestor_record_offset":14892,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":16648,"parent_record_offset":14892,"ancestor_record_offset":14892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":16272,"parent_record_offset":14892,"ancestor_record_offset":14892,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":15108,"parent_record_offset":14892,"ancestor_record_offset":14892,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":15048,"parent_record_offset":14892,"ancestor_record_offset":14892,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":15780,"parent_record_offset":15756,"ancestor_record_offset":14892,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":15756,"parent_record_offset":15700,"ancestor_record_offset":14892,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":15728,"parent_record_offset":15700,"ancestor_record_offset":14892,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":14892,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":14892,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":16232,"parent_record_offset":16192,"ancestor_record_offset":14892,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":16192,"parent_record_offset":16160,"ancestor_record_offset":14892,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":15308,"parent_record_offset":15108,"ancestor_record_offset":14892,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":15532,"parent_record_offset":15108,"ancestor_record_offset":14892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":14892,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":15260,"parent_record_offset":15236,"ancestor_record_offset":14892,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":15236,"parent_record_offset":15212,"ancestor_record_offset":14892,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":14892,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":14892,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":15140,"parent_record_offset":15108,"ancestor_record_offset":14892,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":16964,"rva":3853936,"va":5372563056}
// 编译实例 RVAs: 0x3ace70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e0530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-838149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e8fb35f3ff","size":5,"soft_fail":false,"target_va":5371725104,"va":5372563248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2336045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e82da52300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372564686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2518284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e80c6d2600","size":5,"soft_fail":false,"target_va":5375081488,"va":5372563199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2518211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e8c36c2600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372563288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10264139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e84b9e9c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372563584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38752287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e81f504f02","size":5,"soft_fail":false,"target_va":5411315744,"va":5372563452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3ec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38760101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e8a56e4f02","size":5,"soft_fail":false,"target_va":5411323584,"va":5372563478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38976663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e897bc5202","size":5,"soft_fail":false,"target_va":5411540963,"va":5372564295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10142620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e89cc39a00","size":5,"soft_fail":false,"target_va":5382706224,"va":5372563599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10180933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372563056,"raw_hex":"e845599b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372563142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372563056,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372563822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":17108,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":17136,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":17228,"parent_record_offset":16964,"ancestor_record_offset":16964,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":16964,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":16964,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":16964,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17676,"parent_record_offset":17648,"ancestor_record_offset":16964,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":18024,"parent_record_offset":17968,"ancestor_record_offset":16964,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":16964,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17588,"parent_record_offset":17560,"ancestor_record_offset":16964,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17996,"parent_record_offset":17968,"ancestor_record_offset":16964,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17968,"parent_record_offset":17944,"ancestor_record_offset":16964,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17648,"parent_record_offset":17616,"ancestor_record_offset":16964,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17616,"parent_record_offset":17588,"ancestor_record_offset":16964,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":16964,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":17532,"parent_record_offset":17468,"ancestor_record_offset":16964,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":17560,"parent_record_offset":17532,"ancestor_record_offset":16964,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":16964,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":19624,"parent_record_offset":19588,"ancestor_record_offset":16964,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":16964,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":22392,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":20384,"parent_record_offset":20348,"ancestor_record_offset":16964,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":16964,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":17896,"parent_record_offset":17468,"ancestor_record_offset":16964,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":16964,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20588,"parent_record_offset":20564,"ancestor_record_offset":16964,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":16964,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":20308,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":19220,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":157,"record_offset":17864,"parent_record_offset":17780,"ancestor_record_offset":16964,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17808,"parent_record_offset":17780,"ancestor_record_offset":16964,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17832,"parent_record_offset":17808,"ancestor_record_offset":16964,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17416,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":10610,"source_line":2217,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17468,"parent_record_offset":17416,"ancestor_record_offset":16964,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17780,"parent_record_offset":17468,"ancestor_record_offset":16964,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19736,"parent_record_offset":19708,"ancestor_record_offset":16964,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":16964,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19764,"parent_record_offset":19736,"ancestor_record_offset":16964,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":19588,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20512,"parent_record_offset":20476,"ancestor_record_offset":16964,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20476,"parent_record_offset":20444,"ancestor_record_offset":16964,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":17388,"parent_record_offset":17268,"ancestor_record_offset":16964,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":16964,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19680,"parent_record_offset":19652,"ancestor_record_offset":16964,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19652,"parent_record_offset":19624,"ancestor_record_offset":16964,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":17168,"parent_record_offset":17136,"ancestor_record_offset":16964,"inlinee":13508,"source_line":28,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":17268,"parent_record_offset":17228,"ancestor_record_offset":16964,"inlinee":13504,"source_line":29,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":20348,"parent_record_offset":20308,"ancestor_record_offset":16964,"inlinee":13506,"source_line":39,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":21760,"parent_record_offset":21728,"ancestor_record_offset":16964,"inlinee":13507,"source_line":44,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C4
// 首条位置: {"module":157,"inlinee":13508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":17168,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C0
// 首条位置: {"module":157,"inlinee":13504}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":19192,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":22392,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":19276,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":20308,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":19220,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17416,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":10610,"source_line":2217,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":19588,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":17388,"parent_record_offset":17268,"ancestor_record_offset":17268,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":17268,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":17268,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":17268,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":17676,"parent_record_offset":17648,"ancestor_record_offset":17268,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":18024,"parent_record_offset":17968,"ancestor_record_offset":17268,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":17268,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17588,"parent_record_offset":17560,"ancestor_record_offset":17268,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17996,"parent_record_offset":17968,"ancestor_record_offset":17268,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17968,"parent_record_offset":17944,"ancestor_record_offset":17268,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17648,"parent_record_offset":17616,"ancestor_record_offset":17268,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17616,"parent_record_offset":17588,"ancestor_record_offset":17268,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":17268,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":17532,"parent_record_offset":17468,"ancestor_record_offset":17268,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":17560,"parent_record_offset":17532,"ancestor_record_offset":17268,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":17268,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":19624,"parent_record_offset":19588,"ancestor_record_offset":17268,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":20384,"parent_record_offset":20348,"ancestor_record_offset":17268,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":17268,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":17896,"parent_record_offset":17468,"ancestor_record_offset":17268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":17268,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20588,"parent_record_offset":20564,"ancestor_record_offset":17268,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":17268,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":157,"record_offset":17864,"parent_record_offset":17780,"ancestor_record_offset":17268,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17808,"parent_record_offset":17780,"ancestor_record_offset":17268,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17832,"parent_record_offset":17808,"ancestor_record_offset":17268,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17468,"parent_record_offset":17416,"ancestor_record_offset":17268,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":157,"record_offset":17780,"parent_record_offset":17468,"ancestor_record_offset":17268,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19736,"parent_record_offset":19708,"ancestor_record_offset":17268,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":17268,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":19764,"parent_record_offset":19736,"ancestor_record_offset":17268,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20512,"parent_record_offset":20476,"ancestor_record_offset":17268,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20476,"parent_record_offset":20444,"ancestor_record_offset":17268,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":17268,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19680,"parent_record_offset":19652,"ancestor_record_offset":17268,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":19652,"parent_record_offset":19624,"ancestor_record_offset":17268,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":20348,"parent_record_offset":20308,"ancestor_record_offset":17268,"inlinee":13506,"source_line":39,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":21760,"parent_record_offset":21728,"ancestor_record_offset":17268,"inlinee":13507,"source_line":44,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C2
// 首条位置: {"module":157,"inlinee":13506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":20384,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":20412,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":20852,"parent_record_offset":20828,"ancestor_record_offset":20348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":20828,"parent_record_offset":20744,"ancestor_record_offset":20348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":20348,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":20348,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":20348,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":20444,"parent_record_offset":20412,"ancestor_record_offset":20348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":20348,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20588,"parent_record_offset":20564,"ancestor_record_offset":20348,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":20348,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20512,"parent_record_offset":20476,"ancestor_record_offset":20348,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":20476,"parent_record_offset":20444,"ancestor_record_offset":20348,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C3
// 首条位置: {"module":157,"inlinee":13507}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":21788,"parent_record_offset":21760,"ancestor_record_offset":21760,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":21816,"parent_record_offset":21760,"ancestor_record_offset":21760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":21760,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":22212,"parent_record_offset":22128,"ancestor_record_offset":21760,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":22128,"parent_record_offset":22100,"ancestor_record_offset":21760,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":22156,"parent_record_offset":22128,"ancestor_record_offset":21760,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":21760,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":21844,"parent_record_offset":21816,"ancestor_record_offset":21760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":21928,"parent_record_offset":21900,"ancestor_record_offset":21760,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":21976,"parent_record_offset":21952,"ancestor_record_offset":21760,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":21952,"parent_record_offset":21928,"ancestor_record_offset":21760,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":21760,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":21872,"parent_record_offset":21844,"ancestor_record_offset":21760,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":22912,"rva":3855936,"va":5372565056}
// 编译实例 RVAs: 0x3ad640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10281118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e89ee09c00","size":5,"soft_fail":false,"target_va":5382847888,"va":5372566765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140241ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1490113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e83f43e9ff","size":5,"soft_fail":false,"target_va":5371075744,"va":5372565852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-984446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e882faf0ff","size":5,"soft_fail":false,"target_va":5371581280,"va":5372565721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2333544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8689b2300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372567187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2515309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e86d612600","size":5,"soft_fail":false,"target_va":5375081488,"va":5372566174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2515952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8f0632600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372565547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5340424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8087d5100","size":5,"soft_fail":false,"target_va":5377906176,"va":5372565747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b31cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7881785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e839447800","size":5,"soft_fail":false,"target_va":5380447472,"va":5372565682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10262486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8d6979c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372565237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10274776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8d8c79c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5372565427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38973932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8ecb15202","size":5,"soft_fail":false,"target_va":5411540963,"va":5372567026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10088928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8e0f19900","size":5,"soft_fail":false,"target_va":5382655024,"va":5372566091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38974731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e80bb55202","size":5,"soft_fail":false,"target_va":5411541776,"va":5372567040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::iter_resolved_snapshots
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7172475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e87b716d00","size":5,"soft_fail":false,"target_va":5379738176,"va":5372565696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::HistoryIdentity::generate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7170475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8ab696d00","size":5,"soft_fail":false,"target_va":5379736096,"va":5372565616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5652090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e87a3e5600","size":5,"soft_fail":false,"target_va":5378217744,"va":5372565649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10143409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e8b1c69a00","size":5,"soft_fail":false,"target_va":5382708672,"va":5372565258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10178917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372565056,"raw_hex":"e865519b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372565158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372565056,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372565518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":23056,"parent_record_offset":22912,"ancestor_record_offset":22912,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":23084,"parent_record_offset":22912,"ancestor_record_offset":22912,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":23112,"parent_record_offset":22912,"ancestor_record_offset":22912,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25664,"parent_record_offset":25640,"ancestor_record_offset":22912,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25688,"parent_record_offset":25664,"ancestor_record_offset":22912,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":22912,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25640,"parent_record_offset":25612,"ancestor_record_offset":22912,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24276,"parent_record_offset":24188,"ancestor_record_offset":22912,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":24188,"parent_record_offset":24160,"ancestor_record_offset":22912,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23764,"parent_record_offset":23736,"ancestor_record_offset":22912,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23812,"parent_record_offset":23788,"ancestor_record_offset":22912,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25552,"parent_record_offset":25524,"ancestor_record_offset":22912,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25212,"parent_record_offset":25184,"ancestor_record_offset":22912,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":22912,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25612,"parent_record_offset":25580,"ancestor_record_offset":22912,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25580,"parent_record_offset":25552,"ancestor_record_offset":22912,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25160,"parent_record_offset":25136,"ancestor_record_offset":22912,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27856,"parent_record_offset":27808,"ancestor_record_offset":22912,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":27620,"parent_record_offset":27580,"ancestor_record_offset":22912,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25488,"parent_record_offset":25460,"ancestor_record_offset":22912,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25460,"parent_record_offset":25432,"ancestor_record_offset":22912,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25404,"parent_record_offset":25376,"ancestor_record_offset":22912,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25432,"parent_record_offset":25404,"ancestor_record_offset":22912,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25376,"parent_record_offset":25348,"ancestor_record_offset":22912,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":157,"record_offset":25348,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27552,"parent_record_offset":27504,"ancestor_record_offset":22912,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":25524,"parent_record_offset":25488,"ancestor_record_offset":22912,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27464,"parent_record_offset":27416,"ancestor_record_offset":22912,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":24796,"parent_record_offset":24768,"ancestor_record_offset":22912,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27504,"parent_record_offset":27464,"ancestor_record_offset":22912,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":24220,"parent_record_offset":24188,"ancestor_record_offset":22912,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27880,"parent_record_offset":27856,"ancestor_record_offset":22912,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27904,"parent_record_offset":27880,"ancestor_record_offset":22912,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":23472,"parent_record_offset":23324,"ancestor_record_offset":22912,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":157,"record_offset":23848,"parent_record_offset":23736,"ancestor_record_offset":22912,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":22912,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":27952,"parent_record_offset":27928,"ancestor_record_offset":22912,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":28140,"parent_record_offset":28096,"ancestor_record_offset":22912,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":28096,"parent_record_offset":27620,"ancestor_record_offset":22912,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":157,"record_offset":24768,"parent_record_offset":24740,"ancestor_record_offset":22912,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":28240,"parent_record_offset":27620,"ancestor_record_offset":22912,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":22912,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":26604,"parent_record_offset":26576,"ancestor_record_offset":22912,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":26632,"parent_record_offset":26604,"ancestor_record_offset":22912,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":25244,"parent_record_offset":25212,"ancestor_record_offset":22912,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":27256,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":22912,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":27336,"parent_record_offset":27300,"ancestor_record_offset":22912,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":28288,"parent_record_offset":27416,"ancestor_record_offset":22912,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":25740,"parent_record_offset":25488,"ancestor_record_offset":22912,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":23876,"parent_record_offset":23736,"ancestor_record_offset":22912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":26668,"parent_record_offset":26516,"ancestor_record_offset":22912,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":22912,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":22912,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":22912,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":28200,"parent_record_offset":28168,"ancestor_record_offset":22912,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":23704,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":24684,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":25084,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":28168,"parent_record_offset":28140,"ancestor_record_offset":22912,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":28352,"parent_record_offset":28320,"ancestor_record_offset":22912,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":157,"record_offset":24712,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":13514,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28320,"parent_record_offset":28288,"ancestor_record_offset":22912,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28396,"parent_record_offset":28320,"ancestor_record_offset":22912,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27416,"parent_record_offset":27372,"ancestor_record_offset":22912,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27372,"parent_record_offset":27300,"ancestor_record_offset":22912,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26516,"parent_record_offset":26484,"ancestor_record_offset":22912,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26548,"parent_record_offset":26516,"ancestor_record_offset":22912,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26456,"parent_record_offset":26432,"ancestor_record_offset":22912,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26484,"parent_record_offset":26456,"ancestor_record_offset":22912,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26576,"parent_record_offset":26516,"ancestor_record_offset":22912,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26432,"parent_record_offset":26408,"ancestor_record_offset":22912,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":157,"record_offset":24740,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":4881,"source_line":2348,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23568,"parent_record_offset":23544,"ancestor_record_offset":22912,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23616,"parent_record_offset":23592,"ancestor_record_offset":22912,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23592,"parent_record_offset":23568,"ancestor_record_offset":22912,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23324,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":11592,"source_line":450,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":22912,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23668,"parent_record_offset":23324,"ancestor_record_offset":22912,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":22912,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23932,"parent_record_offset":23904,"ancestor_record_offset":22912,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":157,"record_offset":25916,"parent_record_offset":25892,"ancestor_record_offset":22912,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23296,"parent_record_offset":23144,"ancestor_record_offset":22912,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23544,"parent_record_offset":23520,"ancestor_record_offset":22912,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":22912,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23496,"parent_record_offset":23472,"ancestor_record_offset":22912,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23412,"parent_record_offset":23348,"ancestor_record_offset":22912,"inlinee":11597,"source_line":218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23440,"parent_record_offset":23348,"ancestor_record_offset":22912,"inlinee":11598,"source_line":215,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23384,"parent_record_offset":23348,"ancestor_record_offset":22912,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":23144,"parent_record_offset":23112,"ancestor_record_offset":22912,"inlinee":13510,"source_line":64,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":23736,"parent_record_offset":23704,"ancestor_record_offset":22912,"inlinee":13512,"source_line":69,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":24880,"parent_record_offset":24836,"ancestor_record_offset":22912,"inlinee":13515,"source_line":96,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":27300,"parent_record_offset":27256,"ancestor_record_offset":22912,"inlinee":13516,"source_line":100,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C6
// 首条位置: {"module":157,"inlinee":13510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":157,"record_offset":25348,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":27256,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":25112,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":23704,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":24684,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":25084,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":157,"record_offset":24712,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":13514,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":157,"record_offset":24740,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":4881,"source_line":2348,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23324,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":11592,"source_line":450,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23296,"parent_record_offset":23144,"ancestor_record_offset":23144,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25664,"parent_record_offset":25640,"ancestor_record_offset":23144,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25688,"parent_record_offset":25664,"ancestor_record_offset":23144,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":23144,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":25640,"parent_record_offset":25612,"ancestor_record_offset":23144,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24276,"parent_record_offset":24188,"ancestor_record_offset":23144,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":24188,"parent_record_offset":24160,"ancestor_record_offset":23144,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23764,"parent_record_offset":23736,"ancestor_record_offset":23144,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23812,"parent_record_offset":23788,"ancestor_record_offset":23144,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25552,"parent_record_offset":25524,"ancestor_record_offset":23144,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25212,"parent_record_offset":25184,"ancestor_record_offset":23144,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":23144,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25612,"parent_record_offset":25580,"ancestor_record_offset":23144,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25580,"parent_record_offset":25552,"ancestor_record_offset":23144,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":25160,"parent_record_offset":25136,"ancestor_record_offset":23144,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27856,"parent_record_offset":27808,"ancestor_record_offset":23144,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":27620,"parent_record_offset":27580,"ancestor_record_offset":23144,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25488,"parent_record_offset":25460,"ancestor_record_offset":23144,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25460,"parent_record_offset":25432,"ancestor_record_offset":23144,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25404,"parent_record_offset":25376,"ancestor_record_offset":23144,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25432,"parent_record_offset":25404,"ancestor_record_offset":23144,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":25376,"parent_record_offset":25348,"ancestor_record_offset":23144,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27552,"parent_record_offset":27504,"ancestor_record_offset":23144,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":25524,"parent_record_offset":25488,"ancestor_record_offset":23144,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27464,"parent_record_offset":27416,"ancestor_record_offset":23144,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":24796,"parent_record_offset":24768,"ancestor_record_offset":23144,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27504,"parent_record_offset":27464,"ancestor_record_offset":23144,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":24220,"parent_record_offset":24188,"ancestor_record_offset":23144,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27880,"parent_record_offset":27856,"ancestor_record_offset":23144,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27904,"parent_record_offset":27880,"ancestor_record_offset":23144,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":23472,"parent_record_offset":23324,"ancestor_record_offset":23144,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":157,"record_offset":23848,"parent_record_offset":23736,"ancestor_record_offset":23144,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":23144,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":27952,"parent_record_offset":27928,"ancestor_record_offset":23144,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":28140,"parent_record_offset":28096,"ancestor_record_offset":23144,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":28096,"parent_record_offset":27620,"ancestor_record_offset":23144,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":157,"record_offset":24768,"parent_record_offset":24740,"ancestor_record_offset":23144,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":28240,"parent_record_offset":27620,"ancestor_record_offset":23144,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":23144,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":26604,"parent_record_offset":26576,"ancestor_record_offset":23144,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":26632,"parent_record_offset":26604,"ancestor_record_offset":23144,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":25244,"parent_record_offset":25212,"ancestor_record_offset":23144,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":23144,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":27336,"parent_record_offset":27300,"ancestor_record_offset":23144,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":28288,"parent_record_offset":27416,"ancestor_record_offset":23144,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":25740,"parent_record_offset":25488,"ancestor_record_offset":23144,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":23876,"parent_record_offset":23736,"ancestor_record_offset":23144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":26668,"parent_record_offset":26516,"ancestor_record_offset":23144,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":23144,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23144,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23144,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":28200,"parent_record_offset":28168,"ancestor_record_offset":23144,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":28168,"parent_record_offset":28140,"ancestor_record_offset":23144,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":28352,"parent_record_offset":28320,"ancestor_record_offset":23144,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28320,"parent_record_offset":28288,"ancestor_record_offset":23144,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28396,"parent_record_offset":28320,"ancestor_record_offset":23144,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27416,"parent_record_offset":27372,"ancestor_record_offset":23144,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27372,"parent_record_offset":27300,"ancestor_record_offset":23144,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26516,"parent_record_offset":26484,"ancestor_record_offset":23144,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26548,"parent_record_offset":26516,"ancestor_record_offset":23144,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26456,"parent_record_offset":26432,"ancestor_record_offset":23144,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26484,"parent_record_offset":26456,"ancestor_record_offset":23144,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26576,"parent_record_offset":26516,"ancestor_record_offset":23144,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":157,"record_offset":26432,"parent_record_offset":26408,"ancestor_record_offset":23144,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23568,"parent_record_offset":23544,"ancestor_record_offset":23144,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23616,"parent_record_offset":23592,"ancestor_record_offset":23144,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":23592,"parent_record_offset":23568,"ancestor_record_offset":23144,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":23144,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":23668,"parent_record_offset":23324,"ancestor_record_offset":23144,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":23144,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23932,"parent_record_offset":23904,"ancestor_record_offset":23144,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":157,"record_offset":25916,"parent_record_offset":25892,"ancestor_record_offset":23144,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23544,"parent_record_offset":23520,"ancestor_record_offset":23144,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":23144,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":23496,"parent_record_offset":23472,"ancestor_record_offset":23144,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23412,"parent_record_offset":23348,"ancestor_record_offset":23144,"inlinee":11597,"source_line":218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23440,"parent_record_offset":23348,"ancestor_record_offset":23144,"inlinee":11598,"source_line":215,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":157,"record_offset":23384,"parent_record_offset":23348,"ancestor_record_offset":23144,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":23736,"parent_record_offset":23704,"ancestor_record_offset":23144,"inlinee":13512,"source_line":69,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":24880,"parent_record_offset":24836,"ancestor_record_offset":23144,"inlinee":13515,"source_line":96,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":157,"record_offset":27300,"parent_record_offset":27256,"ancestor_record_offset":23144,"inlinee":13516,"source_line":100,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34C8
// 首条位置: {"module":157,"inlinee":13512}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23764,"parent_record_offset":23736,"ancestor_record_offset":23736,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":157,"record_offset":23848,"parent_record_offset":23736,"ancestor_record_offset":23736,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":23876,"parent_record_offset":23736,"ancestor_record_offset":23736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":23736,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":24276,"parent_record_offset":24188,"ancestor_record_offset":23736,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":24188,"parent_record_offset":24160,"ancestor_record_offset":23736,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":23812,"parent_record_offset":23788,"ancestor_record_offset":23736,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":24220,"parent_record_offset":24188,"ancestor_record_offset":23736,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":157,"record_offset":24244,"parent_record_offset":24220,"ancestor_record_offset":23736,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":23736,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":23904,"parent_record_offset":23876,"ancestor_record_offset":23736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":23736,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23736,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":157,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23736,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23960,"parent_record_offset":23932,"ancestor_record_offset":23736,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":157,"record_offset":23932,"parent_record_offset":23904,"ancestor_record_offset":23736,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34CB
// 首条位置: {"module":157,"inlinee":13515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":24908,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":157,"record_offset":24992,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":25020,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":157,"record_offset":24956,"parent_record_offset":24932,"ancestor_record_offset":24880,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":24880,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34CC
// 首条位置: {"module":157,"inlinee":13516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":27336,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27372,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":27300,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":27300,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":28588,"parent_record_offset":28564,"ancestor_record_offset":27300,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":27996,"parent_record_offset":27808,"ancestor_record_offset":27300,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":28564,"parent_record_offset":28508,"ancestor_record_offset":27300,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27724,"parent_record_offset":27684,"ancestor_record_offset":27300,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":28536,"parent_record_offset":28508,"ancestor_record_offset":27300,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":28508,"parent_record_offset":28484,"ancestor_record_offset":27300,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27808,"parent_record_offset":27764,"ancestor_record_offset":27300,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27764,"parent_record_offset":27724,"ancestor_record_offset":27300,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":28484,"parent_record_offset":28460,"ancestor_record_offset":27300,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":27856,"parent_record_offset":27808,"ancestor_record_offset":27300,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":27620,"parent_record_offset":27580,"ancestor_record_offset":27300,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":27580,"parent_record_offset":27504,"ancestor_record_offset":27300,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27552,"parent_record_offset":27504,"ancestor_record_offset":27300,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27684,"parent_record_offset":27620,"ancestor_record_offset":27300,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27464,"parent_record_offset":27416,"ancestor_record_offset":27300,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":27504,"parent_record_offset":27464,"ancestor_record_offset":27300,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27880,"parent_record_offset":27856,"ancestor_record_offset":27300,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":27904,"parent_record_offset":27880,"ancestor_record_offset":27300,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":27952,"parent_record_offset":27928,"ancestor_record_offset":27300,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":28140,"parent_record_offset":28096,"ancestor_record_offset":27300,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":28096,"parent_record_offset":27620,"ancestor_record_offset":27300,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":28240,"parent_record_offset":27620,"ancestor_record_offset":27300,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":27928,"parent_record_offset":27904,"ancestor_record_offset":27300,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":28288,"parent_record_offset":27416,"ancestor_record_offset":27300,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":28436,"parent_record_offset":27416,"ancestor_record_offset":27300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":28200,"parent_record_offset":28168,"ancestor_record_offset":27300,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":28168,"parent_record_offset":28140,"ancestor_record_offset":27300,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":28352,"parent_record_offset":28320,"ancestor_record_offset":27300,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28320,"parent_record_offset":28288,"ancestor_record_offset":27300,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":28396,"parent_record_offset":28320,"ancestor_record_offset":27300,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":27416,"parent_record_offset":27372,"ancestor_record_offset":27300,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":29492,"rva":3858512,"va":5372567632}
// 编译实例 RVAs: 0x3ae050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2332587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8ab972300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372568144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2513648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8f05a2600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372567851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10260007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8278e9c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372567716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38753194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8aa534f02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372567937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38973743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e82fb15202","size":5,"soft_fail":false,"target_va":5411541776,"va":5372568028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8321717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8b5fa7e00","size":5,"soft_fail":false,"target_va":5380889392,"va":5372567670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10126778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e8ba859a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372567873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10176371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e873479b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372567704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10173771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e84b3d9b00","size":5,"soft_fail":false,"target_va":5382741504,"va":5372567728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10211166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372567632,"raw_hex":"e85ecf9b00","size":5,"soft_fail":false,"target_va":5382778992,"va":5372567821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":30976,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":29860,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":31464,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":31088,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":29924,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29896,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29720,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::settings_file_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/app_config.rs
//     evidence={"module":157,"record_offset":29664,"parent_record_offset":29492,"ancestor_record_offset":29492,"inlinee":13002,"source_line":166,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":29492,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":30572,"parent_record_offset":30516,"ancestor_record_offset":29492,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":30544,"parent_record_offset":30516,"ancestor_record_offset":29492,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":30516,"parent_record_offset":30492,"ancestor_record_offset":29492,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":30492,"parent_record_offset":30468,"ancestor_record_offset":29492,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":31048,"parent_record_offset":31008,"ancestor_record_offset":29492,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":31008,"parent_record_offset":30976,"ancestor_record_offset":29492,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":30124,"parent_record_offset":29924,"ancestor_record_offset":29492,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":30348,"parent_record_offset":29924,"ancestor_record_offset":29492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":29768,"parent_record_offset":29744,"ancestor_record_offset":29492,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":29816,"parent_record_offset":29792,"ancestor_record_offset":29492,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":29792,"parent_record_offset":29768,"ancestor_record_offset":29492,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29688,"parent_record_offset":29664,"ancestor_record_offset":29492,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29744,"parent_record_offset":29720,"ancestor_record_offset":29492,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29980,"parent_record_offset":29956,"ancestor_record_offset":29492,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":29956,"parent_record_offset":29924,"ancestor_record_offset":29492,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":32092,"rva":3859184,"va":5372568304}
// 编译实例 RVAs: 0x3ae2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2331947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e82b952300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372568784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2513003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e86b582600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372568496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10259362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e8a28b9c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372568361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38752549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e825514f02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372568582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38973098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e8aaae5202","size":5,"soft_fail":false,"target_va":5411541776,"va":5372568673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_profile::profile_file_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5669150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e81e815600","size":5,"soft_fail":false,"target_va":5378237504,"va":5372568349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10126133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e835839a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372568518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10175666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e8b2449b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372568409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10173126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e8c63a9b00","size":5,"soft_fail":false,"target_va":5382741504,"va":5372568373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10210521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568304,"raw_hex":"e8d9cc9b00","size":5,"soft_fail":false,"target_va":5382778992,"va":5372568466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":33516,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":32400,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":34004,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":33628,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":32464,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":32436,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":32260,"parent_record_offset":32092,"ancestor_record_offset":32092,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":32092,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":33112,"parent_record_offset":33056,"ancestor_record_offset":32092,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":33084,"parent_record_offset":33056,"ancestor_record_offset":32092,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":33056,"parent_record_offset":33032,"ancestor_record_offset":32092,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":33032,"parent_record_offset":33008,"ancestor_record_offset":32092,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":33588,"parent_record_offset":33548,"ancestor_record_offset":32092,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":33548,"parent_record_offset":33516,"ancestor_record_offset":32092,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":32664,"parent_record_offset":32464,"ancestor_record_offset":32092,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":32888,"parent_record_offset":32464,"ancestor_record_offset":32092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":32092,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":32356,"parent_record_offset":32332,"ancestor_record_offset":32092,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":32092,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":32092,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":32520,"parent_record_offset":32496,"ancestor_record_offset":32092,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":32496,"parent_record_offset":32464,"ancestor_record_offset":32092,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":34632,"rva":3859824,"va":5372568944}
// 编译实例 RVAs: 0x3ae570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2331357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e8dd922300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372569374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2512398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e80e562600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372569101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10258684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e8fc889c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372569039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38751944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e8c84e4f02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372569187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8320532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e814f67e00","size":5,"soft_fail":false,"target_va":5380889536,"va":5372568999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10125528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e8d8809a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372569123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10175081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e869429b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372568994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10209916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372568944,"raw_hex":"e87cca9b00","size":5,"soft_fail":false,"target_va":5382778992,"va":5372569071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":35908,"parent_record_offset":34632,"ancestor_record_offset":34632,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":36396,"parent_record_offset":34632,"ancestor_record_offset":34632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":36020,"parent_record_offset":34632,"ancestor_record_offset":34632,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":34856,"parent_record_offset":34632,"ancestor_record_offset":34632,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":34796,"parent_record_offset":34632,"ancestor_record_offset":34632,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":35528,"parent_record_offset":35504,"ancestor_record_offset":34632,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":35504,"parent_record_offset":35448,"ancestor_record_offset":34632,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":35476,"parent_record_offset":35448,"ancestor_record_offset":34632,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":35448,"parent_record_offset":35424,"ancestor_record_offset":34632,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":35424,"parent_record_offset":35400,"ancestor_record_offset":34632,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":35980,"parent_record_offset":35940,"ancestor_record_offset":34632,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":35940,"parent_record_offset":35908,"ancestor_record_offset":34632,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":35056,"parent_record_offset":34856,"ancestor_record_offset":34632,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":35280,"parent_record_offset":34856,"ancestor_record_offset":34632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":34960,"parent_record_offset":34936,"ancestor_record_offset":34632,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":35008,"parent_record_offset":34984,"ancestor_record_offset":34632,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":34984,"parent_record_offset":34960,"ancestor_record_offset":34632,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":34936,"parent_record_offset":34912,"ancestor_record_offset":34632,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":34632,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":34888,"parent_record_offset":34856,"ancestor_record_offset":34632,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log
// 记录形式: compiled_procedure; instances=1
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":36712,"rva":3860336,"va":5372569456}
// 编译实例 RVAs: 0x3ae770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2330829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e8cd902300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372569902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2511930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e83a542600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372569569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10258199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e817879c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372569524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38751476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e8f44c4f02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372569655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::dump_log
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8618912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e8a0838300","size":5,"soft_fail":false,"target_va":5381188432,"va":5372569515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10125060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e8047f9a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372569591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10136676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e864ac9a00","size":5,"soft_fail":false,"target_va":5382706224,"va":5372569543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10174569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372569456,"raw_hex":"e869409b00","size":5,"soft_fail":false,"target_va":5382744080,"va":5372569506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":37716,"parent_record_offset":36712,"ancestor_record_offset":36712,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":37856,"parent_record_offset":36712,"ancestor_record_offset":36712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":37828,"parent_record_offset":36712,"ancestor_record_offset":36712,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":157,"record_offset":36900,"parent_record_offset":36712,"ancestor_record_offset":36712,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":36872,"parent_record_offset":36712,"ancestor_record_offset":36712,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":36712,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":37440,"parent_record_offset":37388,"ancestor_record_offset":36712,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37224,"parent_record_offset":37200,"ancestor_record_offset":36712,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":36712,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37388,"parent_record_offset":37364,"ancestor_record_offset":36712,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37272,"parent_record_offset":37248,"ancestor_record_offset":36712,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":36712,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":37364,"parent_record_offset":37344,"ancestor_record_offset":36712,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":36712,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":37176,"parent_record_offset":37152,"ancestor_record_offset":36712,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":37128,"parent_record_offset":36900,"ancestor_record_offset":36712,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":37152,"parent_record_offset":37128,"ancestor_record_offset":36712,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":37788,"parent_record_offset":37748,"ancestor_record_offset":36712,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":37748,"parent_record_offset":37716,"ancestor_record_offset":36712,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":157,"record_offset":36932,"parent_record_offset":36900,"ancestor_record_offset":36712,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":37324,"parent_record_offset":36900,"ancestor_record_offset":36712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":37028,"parent_record_offset":37004,"ancestor_record_offset":36712,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":37076,"parent_record_offset":37052,"ancestor_record_offset":36712,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":37052,"parent_record_offset":37028,"ancestor_record_offset":36712,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":37004,"parent_record_offset":36980,"ancestor_record_offset":36712,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":36980,"parent_record_offset":36956,"ancestor_record_offset":36712,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":36956,"parent_record_offset":36932,"ancestor_record_offset":36712,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00157.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_dump.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_dump.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_dump.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
// 信息 ID: fc9ee4485bb677fc581be590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc1c4 -> 0x3ae570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 信息 ID: 1ac299093d6256de101de1bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbf3c -> 0x3ace70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 信息 ID: d0f22657b9e998b94ee2df81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=157, inline_record=17168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 信息 ID: cfcd7d04639a01a8b1614a84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=157, inline_record=17268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
// 信息 ID: 00dc27ae4e06947d162ca168
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=157, inline_record=20348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 信息 ID: 4b55c5c3fb128dc6db446139
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=157, inline_record=21760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 信息 ID: a47848c512799eb50b10e04a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbfde -> 0x3acc70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 信息 ID: 56f336ac0437b02f75ac660f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc122 -> 0x3ae2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log
// 信息 ID: dd68c40da8c3cde94ceb8044
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbe9a -> 0x3ae770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save
// 信息 ID: 22ae6305eb189bc82c03fbfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbdbe -> 0x3ad640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 信息 ID: 3860d5c634b20a470b4244d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=157, inline_record=23144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 信息 ID: aec3cfb8bfc1d4ae867adf23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=157, inline_record=23736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 信息 ID: 78d59df85401624ef66f1992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=157, inline_record=24880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
// 信息 ID: 617087433aae301607a5efa4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=157, inline_record=27300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 信息 ID: 8633f10bd09a733090cfc3e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc080 -> 0x3ae050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
