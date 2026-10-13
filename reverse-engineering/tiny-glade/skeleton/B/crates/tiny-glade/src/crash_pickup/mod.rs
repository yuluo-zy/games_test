// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/crash_pickup/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: aa37d61f3104ab6776320fbc70138c286f9501f28d0b3582dc3fe5dfe1abb98f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 32, 62, 173, 181
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c","version":"0.8.2"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$4::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":62,"record_offset":41948,"rva":1423296,"va":5370132416}
// 编译实例 RVAs: 0x15b7c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12755565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370132416,"raw_hex":"e86da2c200","size":5,"soft_fail":false,"target_va":5382888144,"va":5370132574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$crash_handler..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t6224378","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370132416,"raw_hex":"e9faf95e00","size":5,"soft_fail":false,"target_va":5376356976,"va":5370132593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$minidumper..errors..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t6215439","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370132416,"raw_hex":"e90fd75e00","size":5,"soft_fail":false,"target_va":5376347920,"va":5370132476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t12580889","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370132416,"raw_hex":"e919f8bf00","size":5,"soft_fail":false,"target_va":5382713360,"va":5370132466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":62,"record_offset":42072,"parent_record_offset":41948,"ancestor_record_offset":41948,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$6::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2665
// 首条位置: {"module":181,"inlinee":9829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$8::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":32,"record_offset":19940,"rva":904688,"va":5369613808}
// 编译实例 RVAs: 0xdcdf0, 0x3df4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t13280388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369613808,"raw_hex":"e884a4ca00","size":5,"soft_fail":false,"target_va":5382894288,"va":5369613895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3694
// 首条位置: {"module":173,"inlinee":13972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":173,"record_offset":34180,"parent_record_offset":34152,"ancestor_record_offset":34152,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":34204,"parent_record_offset":34180,"ancestor_record_offset":34152,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":173,"record_offset":34228,"parent_record_offset":34204,"ancestor_record_offset":34152,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FF3
// 首条位置: {"module":62,"inlinee":8179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":34236,"parent_record_offset":34208,"ancestor_record_offset":34208,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":34700,"parent_record_offset":34208,"ancestor_record_offset":34208,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":62,"record_offset":34264,"parent_record_offset":34208,"ancestor_record_offset":34208,"inlinee":8162,"source_line":166,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":34448,"parent_record_offset":34424,"ancestor_record_offset":34208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":34424,"parent_record_offset":34368,"ancestor_record_offset":34208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":34396,"parent_record_offset":34368,"ancestor_record_offset":34208,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":34368,"parent_record_offset":34344,"ancestor_record_offset":34208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":34344,"parent_record_offset":34320,"ancestor_record_offset":34208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":34296,"parent_record_offset":34264,"ancestor_record_offset":34208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":62,"record_offset":33232,"rva":1420384,"va":5370129504}
// 编译实例 RVAs: 0x15ac60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003eda0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1163049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e8d740eeff","size":5,"soft_fail":false,"target_va":5368966560,"va":5370129604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015a130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e8dff3ffff","size":5,"soft_fail":false,"target_va":5370126640,"va":5370129740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4774406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e806da4800","size":5,"soft_fail":false,"target_va":5374904480,"va":5370130069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4951932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e87c8f4b00","size":5,"soft_fail":false,"target_va":5375081488,"va":5370129551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4951806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e8fe8e4b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5370129693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41411184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e870e27702","size":5,"soft_fail":false,"target_va":5411540963,"va":5370129774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41413023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e89fe97702","size":5,"soft_fail":false,"target_va":5411542880,"va":5370129852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::client::Client::send_message_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     evidence={"assembly":"\tcallq\t6203158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129504,"raw_hex":"e816a75e00","size":5,"soft_fail":false,"target_va":5376332832,"va":5370129669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":34168,"parent_record_offset":33232,"ancestor_record_offset":33232,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":62,"record_offset":33376,"parent_record_offset":33232,"ancestor_record_offset":33232,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::ClientHandle::send_raw_message
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":62,"record_offset":34208,"parent_record_offset":33232,"ancestor_record_offset":33232,"inlinee":8179,"source_line":38,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":33600,"parent_record_offset":33576,"ancestor_record_offset":33232,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":33624,"parent_record_offset":33600,"ancestor_record_offset":33232,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":33232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":33576,"parent_record_offset":33548,"ancestor_record_offset":33232,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":33892,"parent_record_offset":33836,"ancestor_record_offset":33232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33488,"parent_record_offset":33460,"ancestor_record_offset":33232,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33864,"parent_record_offset":33836,"ancestor_record_offset":33232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33836,"parent_record_offset":33812,"ancestor_record_offset":33232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33548,"parent_record_offset":33516,"ancestor_record_offset":33232,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33516,"parent_record_offset":33488,"ancestor_record_offset":33232,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":33812,"parent_record_offset":33788,"ancestor_record_offset":33232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":34236,"parent_record_offset":34208,"ancestor_record_offset":33232,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":62,"record_offset":33432,"parent_record_offset":33376,"ancestor_record_offset":33232,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":62,"record_offset":33460,"parent_record_offset":33432,"ancestor_record_offset":33232,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":33764,"parent_record_offset":33376,"ancestor_record_offset":33232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":34700,"parent_record_offset":34208,"ancestor_record_offset":33232,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":62,"record_offset":34264,"parent_record_offset":34208,"ancestor_record_offset":33232,"inlinee":8162,"source_line":166,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":62,"record_offset":33704,"parent_record_offset":33680,"ancestor_record_offset":33232,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":62,"record_offset":33728,"parent_record_offset":33704,"ancestor_record_offset":33232,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":62,"record_offset":33680,"parent_record_offset":33376,"ancestor_record_offset":33232,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x262D
// 首条位置: {"module":62,"inlinee":9773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":21500,"parent_record_offset":21288,"ancestor_record_offset":21288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21468,"parent_record_offset":21288,"ancestor_record_offset":21288,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21328,"parent_record_offset":21288,"ancestor_record_offset":21288,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":21288,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":21724,"parent_record_offset":21668,"ancestor_record_offset":21288,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21696,"parent_record_offset":21668,"ancestor_record_offset":21288,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21668,"parent_record_offset":21644,"ancestor_record_offset":21288,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":21288,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":21288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":21288,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":21288,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":21288,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21352,"parent_record_offset":21328,"ancestor_record_offset":21288,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x265F
// 首条位置: {"module":173,"inlinee":9823}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::crash_pickup::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":173,"record_offset":21288,"parent_record_offset":21256,"ancestor_record_offset":21256,"inlinee":9773,"source_line":54,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":21256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":21724,"parent_record_offset":21668,"ancestor_record_offset":21256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21696,"parent_record_offset":21668,"ancestor_record_offset":21256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21668,"parent_record_offset":21644,"ancestor_record_offset":21256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":21256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":21500,"parent_record_offset":21288,"ancestor_record_offset":21256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":21256,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":21256,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":173,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":21256,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21468,"parent_record_offset":21288,"ancestor_record_offset":21256,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21352,"parent_record_offset":21328,"ancestor_record_offset":21256,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":173,"record_offset":21328,"parent_record_offset":21288,"ancestor_record_offset":21256,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::run_server
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2660
// 首条位置: {"module":173,"inlinee":9824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":27408,"parent_record_offset":27376,"ancestor_record_offset":27376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":27584,"parent_record_offset":27560,"ancestor_record_offset":27376,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":173,"record_offset":27560,"parent_record_offset":27504,"ancestor_record_offset":27376,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":27532,"parent_record_offset":27504,"ancestor_record_offset":27376,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":27504,"parent_record_offset":27480,"ancestor_record_offset":27376,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":173,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":27376,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":173,"record_offset":27432,"parent_record_offset":27408,"ancestor_record_offset":27376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00173.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":62,"record_offset":34740,"rva":1420976,"va":5370130096}
// 编译实例 RVAs: 0x15aeb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1615892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e814a81800","size":5,"soft_fail":false,"target_va":5371748208,"va":5370132311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e70a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4765938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8f2b84800","size":5,"soft_fail":false,"target_va":5374898336,"va":5370132393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7ad0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4770044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8fcc84800","size":5,"soft_fail":false,"target_va":5374900944,"va":5370130895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4839339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8abd74900","size":5,"soft_fail":false,"target_va":5374971568,"va":5370132224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4950710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8b68a4b00","size":5,"soft_fail":false,"target_va":5375081488,"va":5370130773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4951126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8568c4b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5370130373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c10290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11227942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e82653ab00","size":5,"soft_fail":false,"target_va":5381358224,"va":5370130277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12574323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e873debf00","size":5,"soft_fail":false,"target_va":5382704752,"va":5370130424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12697563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8dbbfc100","size":5,"soft_fail":false,"target_va":5382827728,"va":5370130160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41716507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e81b8b7c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370130656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41409465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8b9db7702","size":5,"soft_fail":false,"target_va":5411540992,"va":5370131522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41409460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8b4db7702","size":5,"soft_fail":false,"target_va":5411540963,"va":5370131498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41411399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e847e37702","size":5,"soft_fail":false,"target_va":5411542880,"va":5370131476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_exe
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12571420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e81cd3bf00","size":5,"soft_fail":false,"target_va":5382701984,"va":5370130559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::temp_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12570980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e864d1bf00","size":5,"soft_fail":false,"target_va":5382701136,"va":5370130151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12613732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e86478c000","size":5,"soft_fail":false,"target_va":5382744080,"va":5370130343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::Child::id
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12615389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8dd7ec000","size":5,"soft_fail":false,"target_va":5382746432,"va":5370131038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::Command::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12615093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8b57dc000","size":5,"soft_fail":false,"target_va":5382745984,"va":5370130886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::process::Command::arg
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t12657607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8c723c100","size":5,"soft_fail":false,"target_va":5382788320,"va":5370130708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::process::Command::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12657406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8fe22c100","size":5,"soft_fail":false,"target_va":5382788048,"va":5370130637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::client::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3337953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8e1ee3200","size":5,"soft_fail":false,"target_va":5373469040,"va":5370131082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11228121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130096,"raw_hex":"e8d953ab00","size":5,"soft_fail":false,"target_va":5381358304,"va":5370130178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rbx","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5370130096,"raw_hex":"ffd3","size":2,"soft_fail":false,"va":5370131365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":35052,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":62,"record_offset":36472,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":35956,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":37404,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":39332,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":36500,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":35464,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":36408,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":34912,"parent_record_offset":34740,"ancestor_record_offset":34740,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":34740,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38752,"parent_record_offset":38728,"ancestor_record_offset":34740,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":35404,"parent_record_offset":35380,"ancestor_record_offset":34740,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39616,"parent_record_offset":39580,"ancestor_record_offset":34740,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38704,"parent_record_offset":38676,"ancestor_record_offset":34740,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":35380,"parent_record_offset":35324,"ancestor_record_offset":34740,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":62,"record_offset":39580,"parent_record_offset":39544,"ancestor_record_offset":34740,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38616,"parent_record_offset":38588,"ancestor_record_offset":34740,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":35352,"parent_record_offset":35324,"ancestor_record_offset":34740,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":34740,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38676,"parent_record_offset":38644,"ancestor_record_offset":34740,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38644,"parent_record_offset":38616,"ancestor_record_offset":34740,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":34740,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38552,"parent_record_offset":38520,"ancestor_record_offset":34740,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38520,"parent_record_offset":38488,"ancestor_record_offset":34740,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38456,"parent_record_offset":38424,"ancestor_record_offset":34740,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38488,"parent_record_offset":38456,"ancestor_record_offset":34740,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38424,"parent_record_offset":38392,"ancestor_record_offset":34740,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":62,"record_offset":38392,"parent_record_offset":38360,"ancestor_record_offset":34740,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":35136,"parent_record_offset":35088,"ancestor_record_offset":34740,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38304,"parent_record_offset":37444,"ancestor_record_offset":34740,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":35088,"parent_record_offset":35052,"ancestor_record_offset":34740,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$38::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38332,"parent_record_offset":38304,"ancestor_record_offset":34740,"inlinee":9777,"source_line":2807,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38360,"parent_record_offset":38332,"ancestor_record_offset":34740,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39544,"parent_record_offset":39504,"ancestor_record_offset":34740,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39828,"parent_record_offset":39804,"ancestor_record_offset":34740,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":62,"record_offset":38588,"parent_record_offset":38552,"ancestor_record_offset":34740,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":62,"record_offset":35164,"parent_record_offset":35088,"ancestor_record_offset":34740,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":62,"record_offset":38828,"parent_record_offset":38804,"ancestor_record_offset":34740,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":62,"record_offset":38804,"parent_record_offset":38552,"ancestor_record_offset":34740,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":35228,"parent_record_offset":35088,"ancestor_record_offset":34740,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39724,"parent_record_offset":39696,"ancestor_record_offset":34740,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":35192,"parent_record_offset":35088,"ancestor_record_offset":34740,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":39456,"parent_record_offset":39408,"ancestor_record_offset":34740,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39864,"parent_record_offset":39828,"ancestor_record_offset":34740,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39888,"parent_record_offset":39864,"ancestor_record_offset":34740,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39920,"parent_record_offset":39828,"ancestor_record_offset":34740,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":62,"record_offset":36436,"parent_record_offset":36408,"ancestor_record_offset":34740,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":62,"record_offset":34960,"parent_record_offset":34936,"ancestor_record_offset":34740,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":62,"record_offset":35008,"parent_record_offset":34984,"ancestor_record_offset":34740,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":62,"record_offset":34984,"parent_record_offset":34960,"ancestor_record_offset":34740,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":62,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":34740,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":34936,"parent_record_offset":34912,"ancestor_record_offset":34740,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$65::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":37536,"parent_record_offset":37504,"ancestor_record_offset":34740,"inlinee":5142,"source_line":2091,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":62,"record_offset":38220,"parent_record_offset":37444,"ancestor_record_offset":34740,"inlinee":5201,"source_line":718,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":62,"record_offset":37504,"parent_record_offset":37444,"ancestor_record_offset":34740,"inlinee":5202,"source_line":655,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":62,"record_offset":37444,"parent_record_offset":37404,"ancestor_record_offset":34740,"inlinee":9775,"source_line":89,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":62,"record_offset":39408,"parent_record_offset":39368,"ancestor_record_offset":34740,"inlinee":9778,"source_line":98,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":62,"record_offset":39504,"parent_record_offset":39456,"ancestor_record_offset":34740,"inlinee":9780,"source_line":105,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x262F
// 首条位置: {"module":62,"inlinee":9775}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38304,"parent_record_offset":37444,"ancestor_record_offset":37444,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":62,"record_offset":38220,"parent_record_offset":37444,"ancestor_record_offset":37444,"inlinee":5201,"source_line":718,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":62,"record_offset":37504,"parent_record_offset":37444,"ancestor_record_offset":37444,"inlinee":5202,"source_line":655,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":37444,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38752,"parent_record_offset":38728,"ancestor_record_offset":37444,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":37840,"parent_record_offset":37816,"ancestor_record_offset":37444,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":38704,"parent_record_offset":38676,"ancestor_record_offset":37444,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":37816,"parent_record_offset":37760,"ancestor_record_offset":37444,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38616,"parent_record_offset":38588,"ancestor_record_offset":37444,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":37788,"parent_record_offset":37760,"ancestor_record_offset":37444,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":37444,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38676,"parent_record_offset":38644,"ancestor_record_offset":37444,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":38644,"parent_record_offset":38616,"ancestor_record_offset":37444,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":62,"record_offset":37736,"parent_record_offset":37712,"ancestor_record_offset":37444,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38552,"parent_record_offset":38520,"ancestor_record_offset":37444,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38520,"parent_record_offset":38488,"ancestor_record_offset":37444,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38456,"parent_record_offset":38424,"ancestor_record_offset":37444,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38488,"parent_record_offset":38456,"ancestor_record_offset":37444,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":62,"record_offset":38424,"parent_record_offset":38392,"ancestor_record_offset":37444,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":62,"record_offset":38392,"parent_record_offset":38360,"ancestor_record_offset":37444,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$38::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38332,"parent_record_offset":38304,"ancestor_record_offset":37444,"inlinee":9777,"source_line":2807,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":62,"record_offset":38360,"parent_record_offset":38332,"ancestor_record_offset":37444,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":62,"record_offset":38588,"parent_record_offset":38552,"ancestor_record_offset":37444,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":62,"record_offset":38828,"parent_record_offset":38804,"ancestor_record_offset":37444,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":62,"record_offset":38804,"parent_record_offset":38552,"ancestor_record_offset":37444,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":37592,"parent_record_offset":37504,"ancestor_record_offset":37444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":62,"record_offset":37560,"parent_record_offset":37536,"ancestor_record_offset":37444,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$65::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":62,"record_offset":37536,"parent_record_offset":37504,"ancestor_record_offset":37444,"inlinee":5142,"source_line":2091,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2632
// 首条位置: {"module":62,"inlinee":9778}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":62,"record_offset":39456,"parent_record_offset":39408,"ancestor_record_offset":39408,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39672,"parent_record_offset":39648,"ancestor_record_offset":39408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39696,"parent_record_offset":39672,"ancestor_record_offset":39408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39616,"parent_record_offset":39580,"ancestor_record_offset":39408,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39648,"parent_record_offset":39616,"ancestor_record_offset":39408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":62,"record_offset":39580,"parent_record_offset":39544,"ancestor_record_offset":39408,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39544,"parent_record_offset":39504,"ancestor_record_offset":39408,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39828,"parent_record_offset":39804,"ancestor_record_offset":39408,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39768,"parent_record_offset":39580,"ancestor_record_offset":39408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39724,"parent_record_offset":39696,"ancestor_record_offset":39408,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39864,"parent_record_offset":39828,"ancestor_record_offset":39408,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39888,"parent_record_offset":39864,"ancestor_record_offset":39408,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39920,"parent_record_offset":39828,"ancestor_record_offset":39408,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":62,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":39408,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":62,"record_offset":39504,"parent_record_offset":39456,"ancestor_record_offset":39408,"inlinee":9780,"source_line":105,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2634
// 首条位置: {"module":62,"inlinee":9780}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39544,"parent_record_offset":39504,"ancestor_record_offset":39504,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39804,"parent_record_offset":39504,"ancestor_record_offset":39504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39672,"parent_record_offset":39648,"ancestor_record_offset":39504,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39696,"parent_record_offset":39672,"ancestor_record_offset":39504,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39616,"parent_record_offset":39580,"ancestor_record_offset":39504,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":62,"record_offset":39648,"parent_record_offset":39616,"ancestor_record_offset":39504,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":62,"record_offset":39580,"parent_record_offset":39544,"ancestor_record_offset":39504,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":62,"record_offset":39828,"parent_record_offset":39804,"ancestor_record_offset":39504,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39768,"parent_record_offset":39580,"ancestor_record_offset":39504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":62,"record_offset":39724,"parent_record_offset":39696,"ancestor_record_offset":39504,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39864,"parent_record_offset":39828,"ancestor_record_offset":39504,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39888,"parent_record_offset":39864,"ancestor_record_offset":39504,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":62,"record_offset":39920,"parent_record_offset":39828,"ancestor_record_offset":39504,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00062.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::ClientHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::ClientHandle>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::ClientHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashHandlerHooks
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tiny_glade::crash_pickup::CrashHandlerHooks,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::CrashHandlerHooks>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashReporter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::Error
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::Error>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result::expect<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::result::Result::expect", "arguments": ["tuple$<>", "enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<usize,core::num::error::ParseIntError> > (*)(ref$<str$>)
// 候选参数类型: ["ref$<str$>"]
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_message
// 信息 ID: 854ef18d454425f07ae331fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdb97b -> 0x15ac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598d97 -> 0x15ac60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598e10 -> 0x15ac60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 信息 ID: b0b7419cbf8699f6c316b77a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::ClientHandle::send_message; module=62, inline_record=34208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::new
// 信息 ID: fa908dfe95257d8590e552ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=173, inline_record=21256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::run_server
// 信息 ID: 601fd808b44d340c0c21ee24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=173, inline_record=27376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 信息 ID: b37fc334ee3cb635124f7e6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dcff9 -> 0x15aeb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$1::default
// 信息 ID: 2c82ecbbec6b58cf0ef0d345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=173, inline_record=21288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 信息 ID: d812e707bf4aaeae074b5d70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=62, inline_record=37444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 信息 ID: 512a1c268bf4eba2eaaacc54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=62, inline_record=39408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
// 信息 ID: a0106895f1221630169fc93e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=62, inline_record=39504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$4::fmt
// 信息 ID: b6e6419252ce0e9c61abc42f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$6::from
// 信息 ID: 5886a219183793a9200bba22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::client::start; module=181, inline_record=76136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$8::fmt
// 信息 ID: 580506c2c94840e354469875
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$9::clone
// 信息 ID: 0f0aa3b01ac5356e99bb23fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=173, inline_record=34152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
