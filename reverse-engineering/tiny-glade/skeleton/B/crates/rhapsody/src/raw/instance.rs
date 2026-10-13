// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/instance.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: db04750d16b1758fe3c19f3b1162b90a598fe2197158820b530ec5968ee57f02
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instance.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 683, 708
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::impl$2::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C4C
// 首条位置: {"module":708,"inlinee":40012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::InstanceBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":708,"record_offset":612,"rva":12619536,"va":5381328656}
// 编译实例 RVAs: 0xc08f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1315018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8ca101400","size":5,"soft_fail":false,"target_va":5382644528,"va":5381329505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6247272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e898aca0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381328755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6247777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e89faaa0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381329276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=6
//     target_aliases=69 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5813297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8cf4ba7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381329324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be0ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-170155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e85567fdff","size":5,"soft_fail":false,"target_va":5381163696,"va":5381333846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1508339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8f3031700","size":5,"soft_fail":false,"target_va":5382838224,"va":5381329880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b6410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13292455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8a7d3ca00","size":5,"soft_fail":false,"target_va":5394621456,"va":5381328996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30515657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8c9a1d101","size":5,"soft_fail":false,"target_va":5411847168,"va":5381331506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30208095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e85ff0cc01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381332892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6973310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8829895ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5381328873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30208037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e825f0cc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381332921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t549879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8f7630800","size":5,"soft_fail":false,"target_va":5381881088,"va":5381331204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::debug_utils::Instance::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13364039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e847ebcb00","size":5,"soft_fail":false,"target_va":5394696128,"va":5381332084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::instance::Instance::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13388175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e88f49cc00","size":5,"soft_fail":false,"target_va":5394719632,"va":5381331452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13268315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e85b75ca00","size":5,"soft_fail":false,"target_va":5394597360,"va":5381329040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::from_bytes_until_nul
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1539551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e8df7d1700","size":5,"soft_fail":false,"target_va":5382868720,"va":5381329164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30209877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381328656,"raw_hex":"e855f7cc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381332998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*776(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381328656,"raw_hex":"ff9508030000","size":6,"soft_fail":false,"va":5381331163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":708,"record_offset":10992,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":10960,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":10932,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":756,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":39982,"source_line":109,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":612,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1032,"parent_record_offset":984,"ancestor_record_offset":612,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1604,"parent_record_offset":1512,"ancestor_record_offset":612,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":708,"record_offset":11032,"parent_record_offset":10992,"ancestor_record_offset":612,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":708,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":612,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":3780,"parent_record_offset":3696,"ancestor_record_offset":612,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":612,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":612,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":612,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":612,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":612,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":612,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":612,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4704,"parent_record_offset":4672,"ancestor_record_offset":612,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4672,"parent_record_offset":4644,"ancestor_record_offset":612,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":612,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4740,"parent_record_offset":4704,"ancestor_record_offset":612,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":612,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1180,"parent_record_offset":984,"ancestor_record_offset":612,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":708,"record_offset":6248,"parent_record_offset":6220,"ancestor_record_offset":612,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":612,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":5012,"parent_record_offset":4984,"ancestor_record_offset":612,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":612,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":3752,"parent_record_offset":3696,"ancestor_record_offset":612,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1264,"parent_record_offset":984,"ancestor_record_offset":612,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4588,"parent_record_offset":4548,"ancestor_record_offset":612,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4616,"parent_record_offset":4588,"ancestor_record_offset":612,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":612,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":612,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":5244,"parent_record_offset":5200,"ancestor_record_offset":612,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":612,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":708,"record_offset":5356,"parent_record_offset":5244,"ancestor_record_offset":612,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":708,"record_offset":4984,"parent_record_offset":4548,"ancestor_record_offset":612,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":708,"record_offset":4512,"parent_record_offset":4476,"ancestor_record_offset":612,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":708,"record_offset":4548,"parent_record_offset":4512,"ancestor_record_offset":612,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::entry::Entry::create_instance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":708,"record_offset":6044,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40004,"source_line":252,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::entry::Entry::enumerate_instance_extension_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":708,"record_offset":1848,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":39984,"source_line":277,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::ext::debug_utils::Instance::create_debug_utils_messenger
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":708,"record_offset":8224,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40010,"source_line":83,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::ApplicationInfo::api_version
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5492,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40000,"source_line":1139,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::DebugUtilsMessengerCreateInfoEXT::pfn_user_callback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":8196,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40008,"source_line":19795,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::ExtensionProperties::extension_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":612,"inlinee":39991,"source_line":991,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::InstanceCreateInfo::enabled_extension_names
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5520,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40002,"source_line":1449,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::InstanceCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5548,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40003,"source_line":1459,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$1658::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":8168,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":40006,"source_line":19761,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$25::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5464,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":39998,"source_line":1085,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":612,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":612,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":612,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":708,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":612,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":708,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":612,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":708,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":612,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":708,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":612,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":3012,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":612,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":612,"inlinee":39995,"source_line":93,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":708,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":612,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":708,"record_offset":5156,"parent_record_offset":5096,"ancestor_record_offset":612,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":708,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":612,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":708,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":612,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":708,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":612,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":708,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":612,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":1680,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":1764,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":612,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5200,"parent_record_offset":5156,"ancestor_record_offset":612,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5576,"parent_record_offset":5548,"ancestor_record_offset":612,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":612,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":612,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":612,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":1572,"parent_record_offset":1540,"ancestor_record_offset":612,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":3100,"parent_record_offset":3068,"ancestor_record_offset":612,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":3128,"parent_record_offset":3068,"ancestor_record_offset":612,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1440,"parent_record_offset":984,"ancestor_record_offset":612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":708,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":612,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":612,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2036,"parent_record_offset":1992,"ancestor_record_offset":612,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":612,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1408,"parent_record_offset":1264,"ancestor_record_offset":612,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":1876,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":39985,"source_line":585,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":3808,"parent_record_offset":3696,"ancestor_record_offset":612,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":7552,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":612,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":612,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":612,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":612,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1956,"parent_record_offset":1916,"ancestor_record_offset":612,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":612,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":612,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2068,"parent_record_offset":1992,"ancestor_record_offset":612,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":612,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":612,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":612,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":612,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6340,"parent_record_offset":6248,"ancestor_record_offset":612,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":3156,"parent_record_offset":3068,"ancestor_record_offset":612,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":3044,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":612,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":612,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":2928,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::Instance::extension_names
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":984,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":39983,"source_line":58,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::Instance::layer_names
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":3696,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":39996,"source_line":101,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::impl$1::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":1916,"parent_record_offset":1876,"ancestor_record_offset":612,"inlinee":39987,"source_line":124,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::impl$1::create::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":2156,"parent_record_offset":1956,"ancestor_record_offset":612,"inlinee":39989,"source_line":125,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::InstanceBuilder::debug_markers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x990F
// 首条位置: {"module":683,"inlinee":39183}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::extension_names
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C2F
// 首条位置: {"module":708,"inlinee":39983}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1032,"parent_record_offset":984,"ancestor_record_offset":984,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1180,"parent_record_offset":984,"ancestor_record_offset":984,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1264,"parent_record_offset":984,"ancestor_record_offset":984,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1440,"parent_record_offset":984,"ancestor_record_offset":984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":984,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":984,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":984,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":984,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1604,"parent_record_offset":1512,"ancestor_record_offset":984,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":984,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":984,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":984,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":984,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":984,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":984,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":984,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":984,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":984,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":1572,"parent_record_offset":1540,"ancestor_record_offset":984,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":984,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1408,"parent_record_offset":1264,"ancestor_record_offset":984,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::layer_names
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C3C
// 首条位置: {"module":708,"inlinee":39996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":3780,"parent_record_offset":3696,"ancestor_record_offset":3696,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":3752,"parent_record_offset":3696,"ancestor_record_offset":3696,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4080,"parent_record_offset":3696,"ancestor_record_offset":3696,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":3808,"parent_record_offset":3696,"ancestor_record_offset":3696,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":3696,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":4000,"parent_record_offset":3944,"ancestor_record_offset":3696,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":708,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":3696,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3696,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":3696,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":3696,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3696,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3696,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":3696,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3696,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4120,"parent_record_offset":4080,"ancestor_record_offset":3696,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":3848,"parent_record_offset":3808,"ancestor_record_offset":3696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":4232,"parent_record_offset":4080,"ancestor_record_offset":3696,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::create
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C2E
// 首条位置: {"module":708,"inlinee":39982}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":3220,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::create_instance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":708,"record_offset":6044,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40004,"source_line":252,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::entry::Entry::enumerate_instance_extension_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":708,"record_offset":1848,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":39984,"source_line":277,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Instance::create_debug_utils_messenger
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":708,"record_offset":8224,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40010,"source_line":83,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ApplicationInfo::api_version
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5492,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40000,"source_line":1139,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCreateInfoEXT::pfn_user_callback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":8196,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40008,"source_line":19795,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::InstanceCreateInfo::enabled_extension_names
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5520,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40002,"source_line":1449,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::InstanceCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5548,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40003,"source_line":1459,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1658::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":8168,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":40006,"source_line":19761,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$25::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":5464,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":39998,"source_line":1085,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":3012,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":1680,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":1764,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":6168,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":1876,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":39985,"source_line":585,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":7552,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":3044,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":2928,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::extension_names
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":984,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":39983,"source_line":58,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::layer_names
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":3696,"parent_record_offset":756,"ancestor_record_offset":756,"inlinee":39996,"source_line":101,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":756,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":756,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":756,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1032,"parent_record_offset":984,"ancestor_record_offset":756,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":756,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":1604,"parent_record_offset":1512,"ancestor_record_offset":756,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":708,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":756,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":3780,"parent_record_offset":3696,"ancestor_record_offset":756,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":756,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":756,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":756,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":756,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":756,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":756,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":756,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4704,"parent_record_offset":4672,"ancestor_record_offset":756,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4672,"parent_record_offset":4644,"ancestor_record_offset":756,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":756,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":4740,"parent_record_offset":4704,"ancestor_record_offset":756,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":756,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":1180,"parent_record_offset":984,"ancestor_record_offset":756,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":708,"record_offset":6248,"parent_record_offset":6220,"ancestor_record_offset":756,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":756,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":5012,"parent_record_offset":4984,"ancestor_record_offset":756,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":756,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":3752,"parent_record_offset":3696,"ancestor_record_offset":756,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1264,"parent_record_offset":984,"ancestor_record_offset":756,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4588,"parent_record_offset":4548,"ancestor_record_offset":756,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":4616,"parent_record_offset":4588,"ancestor_record_offset":756,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":756,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":756,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":5244,"parent_record_offset":5200,"ancestor_record_offset":756,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":756,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":708,"record_offset":5356,"parent_record_offset":5244,"ancestor_record_offset":756,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":708,"record_offset":4984,"parent_record_offset":4548,"ancestor_record_offset":756,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":708,"record_offset":4512,"parent_record_offset":4476,"ancestor_record_offset":756,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":708,"record_offset":4548,"parent_record_offset":4512,"ancestor_record_offset":756,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::ExtensionProperties::extension_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":756,"inlinee":39991,"source_line":991,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":756,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":756,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":708,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":756,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":708,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":756,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":708,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":756,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":708,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":756,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":708,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":756,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":756,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":756,"inlinee":39995,"source_line":93,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":708,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":756,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":708,"record_offset":5156,"parent_record_offset":5096,"ancestor_record_offset":756,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":708,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":756,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":708,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":756,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":708,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":756,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":708,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":756,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":756,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5200,"parent_record_offset":5156,"ancestor_record_offset":756,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5576,"parent_record_offset":5548,"ancestor_record_offset":756,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":708,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":756,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":756,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":756,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":1572,"parent_record_offset":1540,"ancestor_record_offset":756,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":3100,"parent_record_offset":3068,"ancestor_record_offset":756,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":3128,"parent_record_offset":3068,"ancestor_record_offset":756,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1440,"parent_record_offset":984,"ancestor_record_offset":756,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":708,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":756,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":756,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2036,"parent_record_offset":1992,"ancestor_record_offset":756,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":756,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":1408,"parent_record_offset":1264,"ancestor_record_offset":756,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":3808,"parent_record_offset":3696,"ancestor_record_offset":756,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":756,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":756,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":756,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":756,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1956,"parent_record_offset":1916,"ancestor_record_offset":756,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":756,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":756,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2068,"parent_record_offset":1992,"ancestor_record_offset":756,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":756,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":756,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":756,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":756,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":6340,"parent_record_offset":6248,"ancestor_record_offset":756,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":3156,"parent_record_offset":3068,"ancestor_record_offset":756,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":756,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":756,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::impl$1::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":1916,"parent_record_offset":1876,"ancestor_record_offset":756,"inlinee":39987,"source_line":124,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::impl$1::create::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":2156,"parent_record_offset":1956,"ancestor_record_offset":756,"inlinee":39989,"source_line":125,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::impl$1::create::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C33
// 首条位置: {"module":708,"inlinee":39987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":2428,"parent_record_offset":1916,"ancestor_record_offset":1916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1956,"parent_record_offset":1916,"ancestor_record_offset":1916,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1916,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":2616,"parent_record_offset":2512,"ancestor_record_offset":1916,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":1916,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":1916,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":1916,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::ExtensionProperties::extension_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":1916,"inlinee":39991,"source_line":991,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":708,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":1916,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":1916,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1916,"inlinee":39995,"source_line":93,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":2584,"parent_record_offset":2548,"ancestor_record_offset":1916,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":1916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1916,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":2036,"parent_record_offset":1992,"ancestor_record_offset":1916,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":1916,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1916,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1916,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1916,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":1916,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":2068,"parent_record_offset":1992,"ancestor_record_offset":1916,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::impl$1::create::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":2156,"parent_record_offset":1956,"ancestor_record_offset":1916,"inlinee":39989,"source_line":125,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::impl$1::create::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C35
// 首条位置: {"module":708,"inlinee":39989}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::ExtensionProperties::extension_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":39991,"source_line":991,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":708,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":2156,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":2156,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":2156,"inlinee":39995,"source_line":93,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":2156,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":2156,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_blocklist
// 记录形式: compiled_procedure; instances=1
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":708,"record_offset":11508,"rva":12625200,"va":5381334320}
// 编译实例 RVAs: 0xc0a530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1512484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e824141700","size":5,"soft_fail":false,"target_va":5382847888,"va":5381335399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t1309997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e82dfd1300","size":5,"soft_fail":false,"target_va":5382644528,"va":5381334526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=18
//     target_aliases=92 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6442728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e818b19dff","size":5,"soft_fail":false,"target_va":5374900736,"va":5381343459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6371400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e8b8c79eff","size":5,"soft_fail":false,"target_va":5374971760,"va":5381343155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6253527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e82994a0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381335026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=18
//     target_aliases=69 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5818361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e80738a7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381334388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1504939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e8abf61600","size":5,"soft_fail":false,"target_va":5382840208,"va":5381335264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6978415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e8918495ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5381335322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::join_generic_copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13381637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e80530cc00","size":5,"soft_fail":false,"target_va":5394716352,"va":5381334710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t30201071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e8efd4cc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381340828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::os::setenv
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t1450071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381334320,"raw_hex":"e857201600","size":5,"soft_fail":false,"target_va":5382784656,"va":5381334580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":20748,"parent_record_offset":11508,"ancestor_record_offset":11508,"inlinee":40019,"source_line":290,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_legacy_blocklist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":15192,"parent_record_offset":11508,"ancestor_record_offset":11508,"inlinee":40017,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_modern_blocklist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":11648,"parent_record_offset":11508,"ancestor_record_offset":11508,"inlinee":40014,"source_line":233,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":11508,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":13376,"parent_record_offset":13320,"ancestor_record_offset":11508,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":708,"record_offset":14424,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":708,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":11508,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":11508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":11508,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":11508,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":11508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":11508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13296,"parent_record_offset":13272,"ancestor_record_offset":11508,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":12684,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":708,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":11508,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":708,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":11508,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":14044,"parent_record_offset":13996,"ancestor_record_offset":11508,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":11508,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12300,"parent_record_offset":12268,"ancestor_record_offset":11508,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12216,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":13996,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12244,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":13480,"parent_record_offset":13452,"ancestor_record_offset":11508,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11508,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":11860,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":21080,"parent_record_offset":21052,"ancestor_record_offset":11508,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":12176,"parent_record_offset":12140,"ancestor_record_offset":11508,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":708,"record_offset":14508,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":13892,"parent_record_offset":13864,"ancestor_record_offset":11508,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":708,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":11508,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":11508,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":11976,"parent_record_offset":11916,"ancestor_record_offset":11508,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":13224,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":708,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11508,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":12556,"parent_record_offset":12504,"ancestor_record_offset":11508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":14164,"parent_record_offset":13996,"ancestor_record_offset":11508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":12104,"parent_record_offset":12064,"ancestor_record_offset":11508,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":21188,"parent_record_offset":21160,"ancestor_record_offset":11508,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":21216,"parent_record_offset":21188,"ancestor_record_offset":11508,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":708,"record_offset":12440,"parent_record_offset":12268,"ancestor_record_offset":11508,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11508,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":20916,"parent_record_offset":20880,"ancestor_record_offset":11508,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12504,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12596,"parent_record_offset":12504,"ancestor_record_offset":11508,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":21132,"parent_record_offset":21104,"ancestor_record_offset":11508,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":11508,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11508,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12004,"parent_record_offset":11916,"ancestor_record_offset":11508,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":11892,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":11916,"parent_record_offset":11892,"ancestor_record_offset":11508,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":11508,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":11776,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":20880,"parent_record_offset":20748,"ancestor_record_offset":11508,"inlinee":39210,"source_line":100,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":22120,"parent_record_offset":20748,"ancestor_record_offset":11508,"inlinee":39211,"source_line":110,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":21052,"parent_record_offset":20916,"ancestor_record_offset":11508,"inlinee":39173,"source_line":101,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":22232,"parent_record_offset":22144,"ancestor_record_offset":11508,"inlinee":39175,"source_line":111,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":12740,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":40016,"source_line":240,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":15376,"parent_record_offset":15192,"ancestor_record_offset":11508,"inlinee":40018,"source_line":322,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":12064,"parent_record_offset":11648,"ancestor_record_offset":11508,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":12140,"parent_record_offset":12104,"ancestor_record_offset":11508,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_modern_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C4E
// 首条位置: {"module":708,"inlinee":40014}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":708,"record_offset":14424,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":708,"record_offset":12684,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12216,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":13996,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12244,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":11860,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":708,"record_offset":14508,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":13224,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12504,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":11892,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":11776,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":12740,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":40016,"source_line":240,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":12064,"parent_record_offset":11648,"ancestor_record_offset":11648,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":11648,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":708,"record_offset":13376,"parent_record_offset":13320,"ancestor_record_offset":11648,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":708,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":11648,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":11648,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":11648,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":11648,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":11648,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":11648,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":708,"record_offset":13296,"parent_record_offset":13272,"ancestor_record_offset":11648,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":708,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":11648,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":14044,"parent_record_offset":13996,"ancestor_record_offset":11648,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":11648,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12300,"parent_record_offset":12268,"ancestor_record_offset":11648,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":13480,"parent_record_offset":13452,"ancestor_record_offset":11648,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":708,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11648,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":12864,"parent_record_offset":12740,"ancestor_record_offset":11648,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":12176,"parent_record_offset":12140,"ancestor_record_offset":11648,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":13892,"parent_record_offset":13864,"ancestor_record_offset":11648,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":708,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":11648,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":11648,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":11976,"parent_record_offset":11916,"ancestor_record_offset":11648,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":11648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":708,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11648,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11648,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":12556,"parent_record_offset":12504,"ancestor_record_offset":11648,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":708,"record_offset":14164,"parent_record_offset":13996,"ancestor_record_offset":11648,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":12104,"parent_record_offset":12064,"ancestor_record_offset":11648,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":708,"record_offset":12440,"parent_record_offset":12268,"ancestor_record_offset":11648,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11648,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":12596,"parent_record_offset":12504,"ancestor_record_offset":11648,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":11648,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11648,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12004,"parent_record_offset":11916,"ancestor_record_offset":11648,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":12896,"parent_record_offset":12740,"ancestor_record_offset":11648,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":11916,"parent_record_offset":11892,"ancestor_record_offset":11648,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12028,"parent_record_offset":12004,"ancestor_record_offset":11648,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12780,"parent_record_offset":12740,"ancestor_record_offset":11648,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":13072,"parent_record_offset":12740,"ancestor_record_offset":11648,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":12140,"parent_record_offset":12104,"ancestor_record_offset":11648,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C50
// 首条位置: {"module":708,"inlinee":40016}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":12864,"parent_record_offset":12740,"ancestor_record_offset":12740,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":12896,"parent_record_offset":12740,"ancestor_record_offset":12740,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":12780,"parent_record_offset":12740,"ancestor_record_offset":12740,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":13072,"parent_record_offset":12740,"ancestor_record_offset":12740,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":13180,"parent_record_offset":13148,"ancestor_record_offset":12740,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":12952,"parent_record_offset":12920,"ancestor_record_offset":12740,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":12980,"parent_record_offset":12920,"ancestor_record_offset":12740,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":13112,"parent_record_offset":13072,"ancestor_record_offset":12740,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":12740,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":12828,"parent_record_offset":12804,"ancestor_record_offset":12740,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":13008,"parent_record_offset":12920,"ancestor_record_offset":12740,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":12920,"parent_record_offset":12896,"ancestor_record_offset":12740,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":13032,"parent_record_offset":13008,"ancestor_record_offset":12740,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":13148,"parent_record_offset":13112,"ancestor_record_offset":12740,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_legacy_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C51
// 首条位置: {"module":708,"inlinee":40017}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":15376,"parent_record_offset":15192,"ancestor_record_offset":15192,"inlinee":40018,"source_line":322,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":15500,"parent_record_offset":15376,"ancestor_record_offset":15192,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":15820,"parent_record_offset":15784,"ancestor_record_offset":15192,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":15192,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":15616,"parent_record_offset":15556,"ancestor_record_offset":15192,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":15748,"parent_record_offset":15708,"ancestor_record_offset":15192,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":15192,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":15192,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15644,"parent_record_offset":15556,"ancestor_record_offset":15192,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":15532,"parent_record_offset":15376,"ancestor_record_offset":15192,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15192,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":15192,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15416,"parent_record_offset":15376,"ancestor_record_offset":15192,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":15708,"parent_record_offset":15376,"ancestor_record_offset":15192,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":15784,"parent_record_offset":15748,"ancestor_record_offset":15192,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C53
// 首条位置: {"module":708,"inlinee":40019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":20880,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":39210,"source_line":100,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":22120,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":39211,"source_line":110,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":21284,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":40018,"source_line":322,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":708,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":20748,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":21408,"parent_record_offset":21284,"ancestor_record_offset":20748,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":21080,"parent_record_offset":21052,"ancestor_record_offset":20748,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":21728,"parent_record_offset":21692,"ancestor_record_offset":20748,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":21496,"parent_record_offset":21464,"ancestor_record_offset":20748,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":21524,"parent_record_offset":21464,"ancestor_record_offset":20748,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":21004,"parent_record_offset":20956,"ancestor_record_offset":20748,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":21656,"parent_record_offset":21616,"ancestor_record_offset":20748,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":21188,"parent_record_offset":21160,"ancestor_record_offset":20748,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":21216,"parent_record_offset":21188,"ancestor_record_offset":20748,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":20916,"parent_record_offset":20880,"ancestor_record_offset":20748,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":20956,"parent_record_offset":20916,"ancestor_record_offset":20748,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":21132,"parent_record_offset":21104,"ancestor_record_offset":20748,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":20748,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":20748,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":21552,"parent_record_offset":21464,"ancestor_record_offset":20748,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":21440,"parent_record_offset":21284,"ancestor_record_offset":20748,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":20748,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":21576,"parent_record_offset":21552,"ancestor_record_offset":20748,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":21324,"parent_record_offset":21284,"ancestor_record_offset":20748,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":21052,"parent_record_offset":20916,"ancestor_record_offset":20748,"inlinee":39173,"source_line":101,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":708,"record_offset":22232,"parent_record_offset":22144,"ancestor_record_offset":20748,"inlinee":39175,"source_line":111,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":21616,"parent_record_offset":21284,"ancestor_record_offset":20748,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":21692,"parent_record_offset":21656,"ancestor_record_offset":20748,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::legacy_disable_layer
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C52
// 首条位置: {"module":708,"inlinee":40018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":15500,"parent_record_offset":15376,"ancestor_record_offset":15376,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":15532,"parent_record_offset":15376,"ancestor_record_offset":15376,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15416,"parent_record_offset":15376,"ancestor_record_offset":15376,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":15708,"parent_record_offset":15376,"ancestor_record_offset":15376,"inlinee":5197,"source_line":361,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":708,"record_offset":15820,"parent_record_offset":15784,"ancestor_record_offset":15376,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":15376,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":15616,"parent_record_offset":15556,"ancestor_record_offset":15376,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":15748,"parent_record_offset":15708,"ancestor_record_offset":15376,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":15376,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":15376,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15644,"parent_record_offset":15556,"ancestor_record_offset":15376,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15376,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":15376,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var::closure$0
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":708,"record_offset":15784,"parent_record_offset":15748,"ancestor_record_offset":15376,"inlinee":5124,"source_line":363,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger
// 记录形式: compiled_procedure; instances=1
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":708,"record_offset":25460,"rva":12635456,"va":5381344576}
// 编译实例 RVAs: 0xc0cd40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1298760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e848d11300","size":5,"soft_fail":false,"target_va":5382644528,"va":5381345763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5829602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e81e0ca7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381345629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30502167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e8176dd101","size":5,"soft_fail":false,"target_va":5411847152,"va":5381344980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924010
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30503537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e87172d101","size":5,"soft_fail":false,"target_va":5411848208,"va":5381344666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::to_str
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1524561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e851431700","size":5,"soft_fail":false,"target_va":5382869248,"va":5381344682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30195646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e8bebfcc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381346125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t30196193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e8e1c1cc01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381346098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_cannot_unwind
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30195979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e80bc1cc01","size":5,"soft_fail":false,"target_va":5411542396,"va":5381346412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30196575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e85fc3cc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381346300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1559303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e807cb1700","size":5,"soft_fail":false,"target_va":5382904128,"va":5381344820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30197105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e871c5cc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381346250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::instance::vulkan_debug_messenger::panic_cold_explicit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t30136303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381344576,"raw_hex":"e8efd7cb01","size":5,"soft_fail":false,"target_va":5411482416,"va":5381346108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::DebugUtilsMessageTypeFlagsEXT::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":708,"record_offset":28312,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":40027,"source_line":32,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCallbackDataEXT::message_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":25884,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":40024,"source_line":19884,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCallbackDataEXT::message_id_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":708,"record_offset":25660,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":40021,"source_line":19866,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":28104,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":708,"record_offset":25784,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":708,"record_offset":25624,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":708,"record_offset":26112,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":12972,"source_line":1328,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":28136,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":28020,"parent_record_offset":25460,"ancestor_record_offset":25460,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":708,"record_offset":27316,"parent_record_offset":26856,"ancestor_record_offset":25460,"inlinee":5002,"source_line":633,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":708,"record_offset":27340,"parent_record_offset":27316,"ancestor_record_offset":25460,"inlinee":5004,"source_line":1765,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":26588,"parent_record_offset":26560,"ancestor_record_offset":25460,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":26216,"parent_record_offset":26160,"ancestor_record_offset":25460,"inlinee":7153,"source_line":1836,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":708,"record_offset":27868,"parent_record_offset":26160,"ancestor_record_offset":25460,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":25684,"parent_record_offset":25660,"ancestor_record_offset":25460,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":708,"record_offset":25712,"parent_record_offset":25684,"ancestor_record_offset":25460,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":708,"record_offset":25740,"parent_record_offset":25712,"ancestor_record_offset":25460,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":708,"record_offset":26680,"parent_record_offset":26388,"ancestor_record_offset":25460,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26784,"parent_record_offset":26728,"ancestor_record_offset":25460,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26616,"parent_record_offset":26560,"ancestor_record_offset":25460,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26560,"parent_record_offset":26532,"ancestor_record_offset":25460,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":25460,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26532,"parent_record_offset":26388,"ancestor_record_offset":25460,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":708,"record_offset":26704,"parent_record_offset":26680,"ancestor_record_offset":25460,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":26640,"parent_record_offset":26616,"ancestor_record_offset":25460,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":708,"record_offset":26808,"parent_record_offset":26784,"ancestor_record_offset":25460,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":708,"record_offset":27044,"parent_record_offset":27012,"ancestor_record_offset":25460,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":28192,"parent_record_offset":28160,"ancestor_record_offset":25460,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":708,"record_offset":28220,"parent_record_offset":28160,"ancestor_record_offset":25460,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":708,"record_offset":27164,"parent_record_offset":27136,"ancestor_record_offset":25460,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":25840,"parent_record_offset":25812,"ancestor_record_offset":25460,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":25460,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":708,"record_offset":27964,"parent_record_offset":27940,"ancestor_record_offset":25460,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":708,"record_offset":26468,"parent_record_offset":26388,"ancestor_record_offset":25460,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":708,"record_offset":26496,"parent_record_offset":26468,"ancestor_record_offset":25460,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":708,"record_offset":27136,"parent_record_offset":27084,"ancestor_record_offset":25460,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":708,"record_offset":27012,"parent_record_offset":26988,"ancestor_record_offset":25460,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":708,"record_offset":26360,"parent_record_offset":26272,"ancestor_record_offset":25460,"inlinee":16918,"source_line":1457,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":708,"record_offset":26244,"parent_record_offset":26160,"ancestor_record_offset":25460,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_contained_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":708,"record_offset":26160,"parent_record_offset":26112,"ancestor_record_offset":25460,"inlinee":16798,"source_line":987,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":708,"record_offset":26856,"parent_record_offset":26272,"ancestor_record_offset":25460,"inlinee":16825,"source_line":1115,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":708,"record_offset":26272,"parent_record_offset":26160,"ancestor_record_offset":25460,"inlinee":16919,"source_line":1167,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":26988,"parent_record_offset":26956,"ancestor_record_offset":25460,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":26956,"parent_record_offset":26916,"ancestor_record_offset":25460,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":25460,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":708,"record_offset":26916,"parent_record_offset":26856,"ancestor_record_offset":25460,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":708,"record_offset":27084,"parent_record_offset":26916,"ancestor_record_offset":25460,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":708,"record_offset":27224,"parent_record_offset":27084,"ancestor_record_offset":25460,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":708,"record_offset":27196,"parent_record_offset":27084,"ancestor_record_offset":25460,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":25460,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":708,"record_offset":28068,"parent_record_offset":28044,"ancestor_record_offset":25460,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":28248,"parent_record_offset":28160,"ancestor_record_offset":25460,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":708,"record_offset":28160,"parent_record_offset":28136,"ancestor_record_offset":25460,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":708,"record_offset":28272,"parent_record_offset":28248,"ancestor_record_offset":25460,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::vulkan_debug_messenger::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":25812,"parent_record_offset":25784,"ancestor_record_offset":25460,"inlinee":40023,"source_line":349,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::vulkan_debug_messenger::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":708,"record_offset":26040,"parent_record_offset":26012,"ancestor_record_offset":25460,"inlinee":40025,"source_line":352,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 349
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C57
// 首条位置: {"module":708,"inlinee":40023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":25840,"parent_record_offset":25812,"ancestor_record_offset":25812,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C59
// 首条位置: {"module":708,"inlinee":40025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":708,"record_offset":26068,"parent_record_offset":26040,"ancestor_record_offset":26040,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00708.jsonl.gz"}
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
// 完整结构及原始类型记录: instance.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::instance::Instance
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::instance::Instance,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::instance::Instance", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::instance::InstanceBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instance.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instance.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::create
// 信息 ID: 274cb8f43aee18cda5c6442f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=708, inline_record=756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::extension_names
// 信息 ID: d7acc6313dab6ae934dd6beb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=708, inline_record=984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::layer_names
// 信息 ID: f777bc07edcfd6ed304ea8db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=708, inline_record=3696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::InstanceBuilder::build
// 信息 ID: 67a2bfb368cd673b8dfb608b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsInstance::new @ 0xbd8251 -> 0xc08f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::InstanceBuilder::debug_markers
// 信息 ID: 8bd132418902a39ca3542a47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=683, inline_record=16176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_blocklist
// 信息 ID: 7aabe1d98b5814643406f304
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x59405e -> 0xc0a530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
// 信息 ID: f2923b69a93a1ece1b947183
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=20748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_legacy_blocklist
// 信息 ID: b7a04adfa56e8182bec88fea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=15192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_modern_blocklist
// 信息 ID: f60666a9243764d8818ca1a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=11648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
// 信息 ID: 72a004dcebe9177a08533e3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=12740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::impl$1::create::closure$1
// 信息 ID: e91ab38d962ec49612b54f0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::instance::impl$1::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=708, inline_record=1916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::impl$1::create::closure$1::closure$0
// 信息 ID: c1f6db598fdf084caa8e41ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::instance::impl$1::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=708, inline_record=2156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::impl$2::default
// 信息 ID: 10b69d483a197046e40bca77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::legacy_disable_layer
// 信息 ID: b2384f88425fa21a8c0c7ab2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=15376; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=15864; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=16352; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=16840; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=17328; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=17816; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=18304; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::apply_vulkan_blocklist; module=708, inline_record=18792; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger
// 信息 ID: 89a00e473310da8c72aa0033
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger::closure$0
// 信息 ID: 9fd2a63911e6d03913c25641
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::instance::vulkan_debug_messenger", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::vulkan_debug_messenger; module=708, inline_record=25812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger::closure$1
// 信息 ID: 9f70d3b41b298bdfad86c18a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::instance::vulkan_debug_messenger", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::vulkan_debug_messenger; module=708, inline_record=26040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
