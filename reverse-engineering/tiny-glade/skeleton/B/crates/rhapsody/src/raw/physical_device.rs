// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/physical_device.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b195f639170bf188f268fd70a34005ac547cd783b6207c163670dce31b430487
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: physical_device.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 140, 240, 685, 687, 703, 704, 936, 1935, 1936, 1988, 2761, 3273
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nvapi; // namespace candidate
//   package identities: [{"name":"nvapi","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.4"}]
// use nvapi_hi; // namespace candidate
//   package identities: [{"name":"nvapi-hi","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":2536,"rva":12582528,"va":5381291648}
// 编译实例 RVAs: 0xbffe80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1555650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e8c2bc1700","size":5,"soft_fail":false,"target_va":5382847888,"va":5381292233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6210682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e8863ba1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381292181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c00200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e806030000","size":5,"soft_fail":false,"target_va":5381292544,"va":5381291765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1600375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e8776b1800","size":5,"soft_fail":false,"target_va":5382892432,"va":5381292052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30555237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e8653cd201","size":5,"soft_fail":false,"target_va":5411847152,"va":5381291910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1548332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e82ca01700","size":5,"soft_fail":false,"target_va":5382840208,"va":5381291871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1595984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e8505a1800","size":5,"soft_fail":false,"target_va":5382888144,"va":5381292155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30250597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381291648,"raw_hex":"e86596cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381292278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":704,"record_offset":3124,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":704,"record_offset":2688,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":704,"record_offset":3676,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":704,"record_offset":3648,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":3096,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":3704,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":704,"record_offset":3208,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":8791,"source_line":1391,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2536,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":3012,"parent_record_offset":2956,"ancestor_record_offset":2536,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":704,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2536,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2536,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2536,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":2536,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":704,"record_offset":2768,"parent_record_offset":2716,"ancestor_record_offset":2536,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":704,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":2536,"inlinee":22273,"source_line":2506,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":704,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":2536,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":704,"record_offset":2796,"parent_record_offset":2716,"ancestor_record_offset":2536,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2536,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":2860,"parent_record_offset":2716,"ancestor_record_offset":2536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":2824,"parent_record_offset":2716,"ancestor_record_offset":2536,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":704,"record_offset":3488,"parent_record_offset":3316,"ancestor_record_offset":2536,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":704,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":2536,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":704,"record_offset":3288,"parent_record_offset":3260,"ancestor_record_offset":2536,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2536,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":2536,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3316,"parent_record_offset":3288,"ancestor_record_offset":2536,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":2536,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":2536,"inlinee":5333,"source_line":547,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":3348,"parent_record_offset":3316,"ancestor_record_offset":2536,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":704,"record_offset":3260,"parent_record_offset":3232,"ancestor_record_offset":2536,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$3::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":5040,"rva":12583424,"va":5381292544}
// 编译实例 RVAs: 0xc00200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1595465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381292544,"raw_hex":"e849581800","size":5,"soft_fail":false,"target_va":5382888144,"va":5381292674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":704,"record_offset":5168,"parent_record_offset":5040,"ancestor_record_offset":5040,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$13::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":148448,"rva":12581600,"va":5381290720}
// 编译实例 RVAs: 0xbffae0, 0x18cd5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1603518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381290720,"raw_hex":"e8be771800","size":5,"soft_fail":false,"target_va":5382894288,"va":5381290765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$16::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BB4
// 首条位置: {"module":703,"record_offset":148568,"rva":12581664,"va":5381290784}
// 编译实例 RVAs: 0xbffb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1601807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381290784,"raw_hex":"e80f711800","size":5,"soft_fail":false,"target_va":5382892704,"va":5381290892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D3C
// 首条位置: {"module":240,"inlinee":15676}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":240,"record_offset":49976,"parent_record_offset":49924,"ancestor_record_offset":49924,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":240,"record_offset":50084,"parent_record_offset":49924,"ancestor_record_offset":49924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::impl$19::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":240,"record_offset":50056,"parent_record_offset":49924,"ancestor_record_offset":49924,"inlinee":15678,"source_line":73,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":50264,"parent_record_offset":50236,"ancestor_record_offset":49924,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":50236,"parent_record_offset":50208,"ancestor_record_offset":49924,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":50208,"parent_record_offset":50180,"ancestor_record_offset":49924,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":50180,"parent_record_offset":50156,"ancestor_record_offset":49924,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":240,"record_offset":50108,"parent_record_offset":50084,"ancestor_record_offset":49924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$18::fmt
// 记录形式: compiled_procedure; instances=3
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":2248,"rva":3402992,"va":5372112112}
// 编译实例 RVAs: 0x33ecf0, 0x1e1b640, 0x21386d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field5_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10781178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112112,"raw_hex":"e8fa81a400","size":5,"soft_fail":false,"target_va":5382893552,"va":5372112369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$19::clone
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D3E
// 首条位置: {"module":240,"inlinee":15678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$21::fmt
// 记录形式: compiled_procedure; instances=3
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":140,"record_offset":8572,"rva":3405136,"va":5372114256}
// 编译实例 RVAs: 0x33f550, 0x1e1b750, 0x21387e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10778060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372114256,"raw_hex":"e8cc75a400","size":5,"soft_fail":false,"target_va":5382892704,"va":5372114639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10779760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372114256,"raw_hex":"e8707ca400","size":5,"soft_fail":false,"target_va":5382894288,"va":5372114523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":4620,"rva":12583360,"va":5381292480}
// 编译实例 RVAs: 0xc001c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::DeviceDriverVersion::is_amd
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":4764,"rva":12583392,"va":5381292512}
// 编译实例 RVAs: 0xc001e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::DeviceDriverVersion::is_apple
// 记录形式: compiled_procedure; instances=1
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":4900,"rva":12583408,"va":5381292528}
// 编译实例 RVAs: 0xc001f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::name
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":5288,"rva":12583936,"va":5381293056}
// 编译实例 RVAs: 0xc00400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6211671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e8a937a1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381293154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6211726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e87237a1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381293225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30553980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e87c37d201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381293183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924010
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30555106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e8e23bd201","size":5,"soft_fail":false,"target_va":5411848208,"va":5381293097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::_$LT$impl$u20$core..ffi..c_str..CStr$GT$::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1547058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e8329b1700","size":5,"soft_fail":false,"target_va":5382840176,"va":5381293113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30247692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293056,"raw_hex":"e80c8bcd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381293266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":704,"record_offset":5528,"parent_record_offset":5288,"ancestor_record_offset":5288,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":704,"record_offset":5432,"parent_record_offset":5288,"ancestor_record_offset":5288,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":6152,"parent_record_offset":5288,"ancestor_record_offset":5288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5288,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5288,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5288,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":5972,"parent_record_offset":5796,"ancestor_record_offset":5288,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":5288,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":5740,"parent_record_offset":5712,"ancestor_record_offset":5288,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5288,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":5288,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":5768,"parent_record_offset":5740,"ancestor_record_offset":5288,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5288,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":5832,"parent_record_offset":5796,"ancestor_record_offset":5288,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":5288,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":5288,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":5596,"parent_record_offset":5556,"ancestor_record_offset":5288,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":5624,"parent_record_offset":5596,"ancestor_record_offset":5288,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":5288,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":5712,"parent_record_offset":5680,"ancestor_record_offset":5288,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":704,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":5288,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":704,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":5288,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":704,"record_offset":5460,"parent_record_offset":5432,"ancestor_record_offset":5288,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":704,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":5288,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":704,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":5288,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":704,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":5288,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":5288,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":704,"record_offset":6072,"parent_record_offset":5680,"ancestor_record_offset":5288,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BDA
// 首条位置: {"module":704,"record_offset":6604,"rva":12584240,"va":5381293360}
// 编译实例 RVAs: 0xc00530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30249382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293360,"raw_hex":"e8a691cd01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381293717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":704,"record_offset":6768,"parent_record_offset":6604,"ancestor_record_offset":6604,"inlinee":39864,"source_line":1555,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":6932,"parent_record_offset":6604,"ancestor_record_offset":6604,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":7168,"parent_record_offset":7136,"ancestor_record_offset":6604,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":704,"record_offset":6796,"parent_record_offset":6768,"ancestor_record_offset":6604,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":6980,"parent_record_offset":6956,"ancestor_record_offset":6604,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":7104,"parent_record_offset":7004,"ancestor_record_offset":6604,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":6604,"inlinee":39866,"source_line":49,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":7208,"parent_record_offset":7104,"ancestor_record_offset":6604,"inlinee":39872,"source_line":53,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":7076,"parent_record_offset":7004,"ancestor_record_offset":6604,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":6604,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":6880,"parent_record_offset":6852,"ancestor_record_offset":6604,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":6604,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":704,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6604,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":704,"record_offset":7136,"parent_record_offset":7104,"ancestor_record_offset":6604,"inlinee":39868,"source_line":163,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BBC
// 首条位置: {"module":704,"inlinee":39868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":7168,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":7272,"rva":12584608,"va":5381293728}
// 编译实例 RVAs: 0xc006a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cdd80
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11741941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293728,"raw_hex":"e80bd54cff","size":5,"soft_fail":false,"target_va":5369552256,"va":5381294192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-144626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293728,"raw_hex":"e80ecbfdff","size":5,"soft_fail":false,"target_va":5381149248,"va":5381293869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13353483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293728,"raw_hex":"e80bc2cb00","size":5,"soft_fail":false,"target_va":5394647264,"va":5381293776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30248956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381293728,"raw_hex":"e8fc8fcd01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381294143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":704,"record_offset":7448,"parent_record_offset":7272,"ancestor_record_offset":7272,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":704,"record_offset":7504,"parent_record_offset":7272,"ancestor_record_offset":7272,"inlinee":39864,"source_line":1555,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":7668,"parent_record_offset":7272,"ancestor_record_offset":7272,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":8164,"parent_record_offset":7272,"ancestor_record_offset":7272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7272,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":8564,"parent_record_offset":8532,"ancestor_record_offset":7272,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":7272,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":8532,"parent_record_offset":8512,"ancestor_record_offset":7272,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":704,"record_offset":8404,"parent_record_offset":8204,"ancestor_record_offset":7272,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":704,"record_offset":8320,"parent_record_offset":8204,"ancestor_record_offset":7272,"inlinee":7984,"source_line":96,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":8204,"parent_record_offset":8184,"ancestor_record_offset":7272,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":7272,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":8064,"parent_record_offset":8032,"ancestor_record_offset":7272,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":704,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":7272,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":7904,"parent_record_offset":7864,"ancestor_record_offset":7272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":7724,"parent_record_offset":7696,"ancestor_record_offset":7272,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":8000,"parent_record_offset":7780,"ancestor_record_offset":7272,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":704,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":7272,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":704,"record_offset":7780,"parent_record_offset":7752,"ancestor_record_offset":7272,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":7936,"parent_record_offset":7864,"ancestor_record_offset":7272,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":7864,"parent_record_offset":7828,"ancestor_record_offset":7272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":7828,"parent_record_offset":7780,"ancestor_record_offset":7272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":7696,"parent_record_offset":7668,"ancestor_record_offset":7272,"inlinee":39866,"source_line":49,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":8104,"parent_record_offset":8000,"ancestor_record_offset":7272,"inlinee":39872,"source_line":53,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":704,"record_offset":8488,"parent_record_offset":8204,"ancestor_record_offset":7272,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":704,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7272,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":7272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":8236,"parent_record_offset":8204,"ancestor_record_offset":7272,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":8184,"parent_record_offset":8164,"ancestor_record_offset":7272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":7272,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":7272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":7272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":7616,"parent_record_offset":7588,"ancestor_record_offset":7272,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":7272,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":7272,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7272,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":8624,"parent_record_offset":8564,"ancestor_record_offset":7272,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":8656,"parent_record_offset":8532,"ancestor_record_offset":7272,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":704,"record_offset":8032,"parent_record_offset":8000,"ancestor_record_offset":7272,"inlinee":39874,"source_line":181,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BC2
// 首条位置: {"module":704,"inlinee":39874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":8064,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BDC
// 首条位置: {"module":1935,"inlinee":39900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":5364,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1935,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":5364,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1935,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":5364,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1935,"record_offset":5516,"parent_record_offset":5460,"ancestor_record_offset":5364,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1935,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":5364,"inlinee":39866,"source_line":49,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1935,"record_offset":5620,"parent_record_offset":5516,"ancestor_record_offset":5364,"inlinee":39872,"source_line":53,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1935,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":5364,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1935,"record_offset":5548,"parent_record_offset":5516,"ancestor_record_offset":5364,"inlinee":39876,"source_line":195,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BC4
// 首条位置: {"module":704,"inlinee":39876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1935,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":8712,"rva":12585104,"va":5381294224}
// 编译实例 RVAs: 0xc00890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cdd80
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11742437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294224,"raw_hex":"e81bd34cff","size":5,"soft_fail":false,"target_va":5369552256,"va":5381294688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-145122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294224,"raw_hex":"e81ec9fdff","size":5,"soft_fail":false,"target_va":5381149248,"va":5381294365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13352987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294224,"raw_hex":"e81bc0cb00","size":5,"soft_fail":false,"target_va":5394647264,"va":5381294272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30248460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294224,"raw_hex":"e80c8ecd01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381294639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":704,"record_offset":8880,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":704,"record_offset":8936,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":39864,"source_line":1555,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":9100,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":9596,"parent_record_offset":8712,"ancestor_record_offset":8712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":8712,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":9996,"parent_record_offset":9964,"ancestor_record_offset":8712,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":8712,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":9964,"parent_record_offset":9944,"ancestor_record_offset":8712,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":704,"record_offset":9836,"parent_record_offset":9636,"ancestor_record_offset":8712,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":704,"record_offset":9752,"parent_record_offset":9636,"ancestor_record_offset":8712,"inlinee":7984,"source_line":96,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":9636,"parent_record_offset":9616,"ancestor_record_offset":8712,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":8712,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":9496,"parent_record_offset":9464,"ancestor_record_offset":8712,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":704,"record_offset":8964,"parent_record_offset":8936,"ancestor_record_offset":8712,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":9336,"parent_record_offset":9296,"ancestor_record_offset":8712,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":9156,"parent_record_offset":9128,"ancestor_record_offset":8712,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":704,"record_offset":9432,"parent_record_offset":9212,"ancestor_record_offset":8712,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":704,"record_offset":9184,"parent_record_offset":9156,"ancestor_record_offset":8712,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":704,"record_offset":9212,"parent_record_offset":9184,"ancestor_record_offset":8712,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":9368,"parent_record_offset":9296,"ancestor_record_offset":8712,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":9296,"parent_record_offset":9260,"ancestor_record_offset":8712,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":704,"record_offset":9260,"parent_record_offset":9212,"ancestor_record_offset":8712,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":9128,"parent_record_offset":9100,"ancestor_record_offset":8712,"inlinee":39866,"source_line":49,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":704,"record_offset":9536,"parent_record_offset":9432,"ancestor_record_offset":8712,"inlinee":39872,"source_line":53,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":704,"record_offset":9920,"parent_record_offset":9636,"ancestor_record_offset":8712,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":704,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":8712,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":8712,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":9668,"parent_record_offset":9636,"ancestor_record_offset":8712,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":9616,"parent_record_offset":9596,"ancestor_record_offset":8712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":10028,"parent_record_offset":9996,"ancestor_record_offset":8712,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":8712,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":8992,"parent_record_offset":8964,"ancestor_record_offset":8712,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":9048,"parent_record_offset":9020,"ancestor_record_offset":8712,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":704,"record_offset":9020,"parent_record_offset":8992,"ancestor_record_offset":8712,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":8712,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":8712,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":10056,"parent_record_offset":9996,"ancestor_record_offset":8712,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":10088,"parent_record_offset":9964,"ancestor_record_offset":8712,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":704,"record_offset":9464,"parent_record_offset":9432,"ancestor_record_offset":8712,"inlinee":39878,"source_line":213,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BC6
// 首条位置: {"module":704,"inlinee":39878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":704,"record_offset":9496,"parent_record_offset":9464,"ancestor_record_offset":9464,"inlinee":39870,"source_line":32,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::performance_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":10144,"rva":12585600,"va":5381294720}
// 编译实例 RVAs: 0xc00a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6325140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e86c7c9fff","size":5,"soft_fail":false,"target_va":5374971760,"va":5381296895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6213669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8db2fa1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381295152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6214642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e80e2ca1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381296141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde8a0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-142148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8bcd4fdff","size":5,"soft_fail":false,"target_va":5381154976,"va":5381297119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13365877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e875f2cb00","size":5,"soft_fail":false,"target_va":5394661136,"va":5381295254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t1545252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e824941700","size":5,"soft_fail":false,"target_va":5382840208,"va":5381294951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-6938105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8072296ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5381295012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30244102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8067dcd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381296856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::join_generic_copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13419630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e86ec4cc00","size":5,"soft_fail":false,"target_va":5394716352,"va":5381296717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::clock_frequencies
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13307732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8540fcb00","size":5,"soft_fail":false,"target_va":5394602560,"va":5381294823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::current_pstate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13307651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8030fcb00","size":5,"soft_fail":false,"target_va":5394602928,"va":5381295272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::dynamic_pstates_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13307717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8450fcb00","size":5,"soft_fail":false,"target_va":5394603216,"va":5381295494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::memory_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13306060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8cc08cb00","size":5,"soft_fail":false,"target_va":5394602224,"va":5381296159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::power_usage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13307705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8390fcb00","size":5,"soft_fail":false,"target_va":5394603584,"va":5381295874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::thermal_settings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13307539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381294720,"raw_hex":"e8930ecb00","size":5,"soft_fail":false,"target_va":5394603952,"va":5381296408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":704,"record_offset":10344,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":14992,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":10316,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":10456,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":704,"record_offset":11080,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":704,"record_offset":10428,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":10884,"parent_record_offset":10144,"ancestor_record_offset":10144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10144,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":10144,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":10144,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":11444,"parent_record_offset":11416,"ancestor_record_offset":10144,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":10804,"parent_record_offset":10748,"ancestor_record_offset":10144,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":704,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10144,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":704,"record_offset":11040,"parent_record_offset":10912,"ancestor_record_offset":10144,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":704,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":10144,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":704,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":10144,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10536,"parent_record_offset":10504,"ancestor_record_offset":10144,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":11356,"parent_record_offset":11328,"ancestor_record_offset":10144,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10776,"parent_record_offset":10748,"ancestor_record_offset":10144,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":10144,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":10144,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":10144,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":11416,"parent_record_offset":11384,"ancestor_record_offset":10144,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":11384,"parent_record_offset":11356,"ancestor_record_offset":10144,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":10144,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":10144,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":11264,"parent_record_offset":11236,"ancestor_record_offset":10144,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":11208,"parent_record_offset":11164,"ancestor_record_offset":10144,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":11236,"parent_record_offset":11208,"ancestor_record_offset":10144,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":704,"record_offset":11164,"parent_record_offset":11136,"ancestor_record_offset":10144,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":704,"record_offset":15016,"parent_record_offset":14992,"ancestor_record_offset":10144,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":704,"record_offset":11136,"parent_record_offset":11108,"ancestor_record_offset":10144,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":704,"record_offset":11108,"parent_record_offset":11080,"ancestor_record_offset":10144,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":10504,"parent_record_offset":10456,"ancestor_record_offset":10144,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":11328,"parent_record_offset":11292,"ancestor_record_offset":10144,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":10144,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":704,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":10144,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":13572,"parent_record_offset":13540,"ancestor_record_offset":10144,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":10144,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":704,"record_offset":11544,"parent_record_offset":11292,"ancestor_record_offset":10144,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":10652,"parent_record_offset":10456,"ancestor_record_offset":10144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10144,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":10624,"parent_record_offset":10456,"ancestor_record_offset":10144,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
// 记录形式: compiled_procedure; instances=1
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":16064,"rva":12588576,"va":5381297696}
// 编译实例 RVAs: 0xc01620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6217935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e8311fa1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381299418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-148967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e819bafdff","size":5,"soft_fail":false,"target_va":5381149248,"va":5381298210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30547767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e8371fd201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381299396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30549255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e80725d201","size":5,"soft_fail":false,"target_va":5411847200,"va":5381297940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30241288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e80872cd01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381299699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13349248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e880b1cb00","size":5,"soft_fail":false,"target_va":5394647264,"va":5381298011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30242375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e84776cd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381299684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1449666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381297696,"raw_hex":"e8c21e1600","size":5,"soft_fail":false,"target_va":5382747616,"va":5381297945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*216(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381297696,"raw_hex":"ff90d8000000","size":6,"soft_fail":false,"va":5381297972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":17792,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":704,"record_offset":16632,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":704,"record_offset":18008,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":16234,"source_line":461,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_memory_properties2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":704,"record_offset":16604,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":39883,"source_line":238,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties2::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":704,"record_offset":16400,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":39882,"source_line":11393,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2404::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":704,"record_offset":16256,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":39880,"source_line":27480,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":17428,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":17252,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":17340,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":7253,"source_line":1859,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":16688,"parent_record_offset":16064,"ancestor_record_offset":16064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":17904,"parent_record_offset":17880,"ancestor_record_offset":16064,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":16064,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":16064,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":17880,"parent_record_offset":17848,"ancestor_record_offset":16064,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":704,"record_offset":17820,"parent_record_offset":17792,"ancestor_record_offset":16064,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":16064,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":16064,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":16064,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":17108,"parent_record_offset":17072,"ancestor_record_offset":16064,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":16064,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":704,"record_offset":17072,"parent_record_offset":17048,"ancestor_record_offset":16064,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":704,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":16064,"inlinee":16235,"source_line":466,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":704,"record_offset":16940,"parent_record_offset":16736,"ancestor_record_offset":16064,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":704,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":16064,"inlinee":7948,"source_line":81,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":704,"record_offset":16856,"parent_record_offset":16736,"ancestor_record_offset":16064,"inlinee":7984,"source_line":96,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":704,"record_offset":18176,"parent_record_offset":18032,"ancestor_record_offset":16064,"inlinee":7988,"source_line":121,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":16736,"parent_record_offset":16712,"ancestor_record_offset":16064,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":16064,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":704,"record_offset":18120,"parent_record_offset":18032,"ancestor_record_offset":16064,"inlinee":8000,"source_line":196,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":704,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":16064,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":17612,"parent_record_offset":17428,"ancestor_record_offset":16064,"inlinee":9736,"source_line":1826,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":17700,"parent_record_offset":17428,"ancestor_record_offset":16064,"inlinee":7225,"source_line":1824,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":704,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":16064,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":704,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":16064,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":704,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":16064,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":704,"record_offset":16524,"parent_record_offset":16500,"ancestor_record_offset":16064,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":16428,"parent_record_offset":16400,"ancestor_record_offset":16064,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":16500,"parent_record_offset":16476,"ancestor_record_offset":16064,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":704,"record_offset":18320,"parent_record_offset":18008,"ancestor_record_offset":16064,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":704,"record_offset":17024,"parent_record_offset":16736,"ancestor_record_offset":16064,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":704,"record_offset":16308,"parent_record_offset":16284,"ancestor_record_offset":16064,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":704,"record_offset":16796,"parent_record_offset":16772,"ancestor_record_offset":16064,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":704,"record_offset":16284,"parent_record_offset":16256,"ancestor_record_offset":16064,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":704,"record_offset":16772,"parent_record_offset":16736,"ancestor_record_offset":16064,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":704,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":16064,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":704,"record_offset":18272,"parent_record_offset":18248,"ancestor_record_offset":16064,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":16064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":704,"record_offset":16332,"parent_record_offset":16308,"ancestor_record_offset":16064,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":17140,"parent_record_offset":17108,"ancestor_record_offset":16064,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":17956,"parent_record_offset":17928,"ancestor_record_offset":16064,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":16064,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":18064,"parent_record_offset":18032,"ancestor_record_offset":16064,"inlinee":11099,"source_line":1628,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":16064,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":18404,"parent_record_offset":18368,"ancestor_record_offset":16064,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":16064,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":17168,"parent_record_offset":17108,"ancestor_record_offset":16064,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":16064,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":17200,"parent_record_offset":17072,"ancestor_record_offset":16064,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$5::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x997B
// 首条位置: {"module":687,"inlinee":39291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":687,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":5536,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1988,"record_offset":1408,"rva":26006736,"va":5394715856}
// 编译实例 RVAs: 0x18cd4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19634459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e8e566d4fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394715942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19634559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e88166d4fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394716058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16824865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715856,"raw_hex":"e821ba0001","size":5,"soft_fail":false,"target_va":5411540963,"va":5394716093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*72(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394715856,"raw_hex":"41ff5648","size":4,"soft_fail":false,"va":5394715896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1988,"record_offset":1704,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":2196,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1988,"record_offset":2112,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1988,"record_offset":1672,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":87502,"source_line":336,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1408,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2008,"parent_record_offset":1832,"ancestor_record_offset":1408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":2324,"parent_record_offset":2268,"ancestor_record_offset":1408,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1768,"parent_record_offset":1736,"ancestor_record_offset":1408,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2296,"parent_record_offset":2268,"ancestor_record_offset":1408,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1408,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":1408,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1800,"parent_record_offset":1768,"ancestor_record_offset":1408,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1408,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1868,"parent_record_offset":1832,"ancestor_record_offset":1408,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1988,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":1408,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1988,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1408,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1988,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1408,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1988,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1408,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1988,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1408,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":18508,"rva":12590592,"va":5381299712}
// 编译实例 RVAs: 0xc01e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418cd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13416122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299712,"raw_hex":"e8bab6cc00","size":5,"soft_fail":false,"target_va":5394715856,"va":5381299729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::adapters::try_process
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13399964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299712,"raw_hex":"e89c77cc00","size":5,"soft_fail":false,"target_va":5394699792,"va":5381299823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":18724,"parent_record_offset":18508,"ancestor_record_offset":18508,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":704,"record_offset":18780,"parent_record_offset":18508,"ancestor_record_offset":18508,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":18664,"parent_record_offset":18508,"ancestor_record_offset":18508,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":18696,"parent_record_offset":18508,"ancestor_record_offset":18508,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":704,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":18508,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":704,"record_offset":18748,"parent_record_offset":18724,"ancestor_record_offset":18508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":18508,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155CE
// 首条位置: {"module":1988,"inlinee":87502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBC4
// 首条位置: {"module":3273,"inlinee":113604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":13956,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":6053,"source_line":641,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3273,"record_offset":19036,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":12392,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":3273,"record_offset":19124,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":8006,"source_line":691,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::enumerate_device_extension_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":12532,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113612,"source_line":504,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_memory_properties2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":18040,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39883,"source_line":238,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":8876,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":40429,"source_line":456,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_properties2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":10544,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39697,"source_line":161,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_queue_family_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":12004,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113611,"source_line":467,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties2::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":18324,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39882,"source_line":11393,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":18068,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39864,"source_line":1555,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceProperties2::push_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9928,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39696,"source_line":11108,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceProperties::device_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9020,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":76151,"source_line":937,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1741::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9280,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113606,"source_line":20882,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2404::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":16380,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39880,"source_line":27480,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$652::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9604,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39690,"source_line":11084,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$677::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":16524,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113615,"source_line":11369,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$700::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9456,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113610,"source_line":11601,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$747::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9308,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":113608,"source_line":12023,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3273,"record_offset":8988,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3273,"record_offset":18232,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":12448,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":12924,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::inspect_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":12560,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":13626,"source_line":899,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":18556,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":18952,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":16816,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":9076,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3273,"record_offset":13816,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":9104,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":8904,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::parse_driver_version
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3273,"record_offset":11912,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39908,"source_line":581,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::string_from_c_array
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3273,"record_offset":10572,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":39907,"source_line":524,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":3273,"record_offset":16852,"parent_record_offset":8544,"ancestor_record_offset":8544,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":8544,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":8544,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":8544,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":19208,"parent_record_offset":19180,"ancestor_record_offset":8544,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11060,"parent_record_offset":10884,"ancestor_record_offset":8544,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":13088,"parent_record_offset":12996,"ancestor_record_offset":8544,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3273,"record_offset":19180,"parent_record_offset":19152,"ancestor_record_offset":8544,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":8544,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":13024,"parent_record_offset":12996,"ancestor_record_offset":8544,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":12996,"parent_record_offset":12972,"ancestor_record_offset":8544,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10884,"parent_record_offset":10852,"ancestor_record_offset":8544,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":8544,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":8544,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10920,"parent_record_offset":10884,"ancestor_record_offset":8544,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10760,"parent_record_offset":10732,"ancestor_record_offset":8544,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10732,"parent_record_offset":10704,"ancestor_record_offset":8544,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10676,"parent_record_offset":10648,"ancestor_record_offset":8544,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":8544,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10648,"parent_record_offset":10620,"ancestor_record_offset":8544,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3273,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":8544,"inlinee":6055,"source_line":75,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3273,"record_offset":10620,"parent_record_offset":10572,"ancestor_record_offset":8544,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3273,"record_offset":19152,"parent_record_offset":19124,"ancestor_record_offset":8544,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3273,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":8544,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":12104,"parent_record_offset":12032,"ancestor_record_offset":8544,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":10796,"parent_record_offset":10760,"ancestor_record_offset":8544,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":8544,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":8544,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::instance::impl$3::get_physical_device_queue_family_properties::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3273,"record_offset":12076,"parent_record_offset":12032,"ancestor_record_offset":8544,"inlinee":86353,"source_line":471,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::prelude::read_into_uninitialized_vector
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":12032,"parent_record_offset":12004,"ancestor_record_offset":8544,"inlinee":86362,"source_line":45,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$15::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9632,"parent_record_offset":9604,"ancestor_record_offset":8544,"inlinee":39692,"source_line":889,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$345::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":9776,"parent_record_offset":9632,"ancestor_record_offset":8544,"inlinee":39694,"source_line":6825,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$50::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3273,"record_offset":16552,"parent_record_offset":16524,"ancestor_record_offset":8544,"inlinee":113617,"source_line":1528,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":3273,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":8544,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":3273,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":8544,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3273,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":8544,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3273,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":8544,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":3273,"record_offset":18096,"parent_record_offset":18068,"ancestor_record_offset":8544,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3273,"record_offset":15552,"parent_record_offset":15520,"ancestor_record_offset":8544,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3273,"record_offset":15612,"parent_record_offset":15580,"ancestor_record_offset":8544,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3273,"record_offset":12708,"parent_record_offset":12600,"ancestor_record_offset":8544,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3273,"record_offset":17684,"parent_record_offset":17656,"ancestor_record_offset":8544,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3273,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":8544,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3273,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":8544,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":8544,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":17860,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":8544,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":9412,"parent_record_offset":9388,"ancestor_record_offset":8544,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3273,"record_offset":15968,"parent_record_offset":15804,"ancestor_record_offset":8544,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":3273,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":8544,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3273,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":8544,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3273,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":8544,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3273,"record_offset":18268,"parent_record_offset":18232,"ancestor_record_offset":8544,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3273,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":8544,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":15940,"parent_record_offset":15804,"ancestor_record_offset":8544,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":8544,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":8544,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3273,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":8544,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3273,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":8544,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3273,"record_offset":9340,"parent_record_offset":9308,"ancestor_record_offset":8544,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3273,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":8544,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3273,"record_offset":13056,"parent_record_offset":13024,"ancestor_record_offset":8544,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3273,"record_offset":15580,"parent_record_offset":15520,"ancestor_record_offset":8544,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3273,"record_offset":17416,"parent_record_offset":17388,"ancestor_record_offset":8544,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3273,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":8544,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3273,"record_offset":15156,"parent_record_offset":15120,"ancestor_record_offset":8544,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3273,"record_offset":15272,"parent_record_offset":15244,"ancestor_record_offset":8544,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":9160,"parent_record_offset":9128,"ancestor_record_offset":8544,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":9188,"parent_record_offset":9128,"ancestor_record_offset":8544,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3273,"record_offset":11160,"parent_record_offset":10760,"ancestor_record_offset":8544,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3273,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":8544,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":8544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":8544,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":15360,"parent_record_offset":15244,"ancestor_record_offset":8544,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":8544,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":8544,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":8544,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3273,"record_offset":16040,"parent_record_offset":16016,"ancestor_record_offset":8544,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":15436,"parent_record_offset":15408,"ancestor_record_offset":8544,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":19316,"parent_record_offset":19288,"ancestor_record_offset":8544,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":8544,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":15464,"parent_record_offset":15408,"ancestor_record_offset":8544,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3273,"record_offset":17768,"parent_record_offset":17740,"ancestor_record_offset":8544,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3273,"record_offset":17796,"parent_record_offset":17768,"ancestor_record_offset":8544,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3273,"record_offset":18124,"parent_record_offset":18096,"ancestor_record_offset":8544,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3273,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":8544,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3273,"record_offset":18152,"parent_record_offset":18124,"ancestor_record_offset":8544,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3273,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":8544,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3273,"record_offset":16016,"parent_record_offset":15992,"ancestor_record_offset":8544,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3273,"record_offset":17712,"parent_record_offset":17684,"ancestor_record_offset":8544,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":19064,"parent_record_offset":19036,"ancestor_record_offset":8544,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":20492,"parent_record_offset":20456,"ancestor_record_offset":8544,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":8544,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":19088,"parent_record_offset":19064,"ancestor_record_offset":8544,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":8544,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":20516,"parent_record_offset":20492,"ancestor_record_offset":8544,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":20548,"parent_record_offset":20456,"ancestor_record_offset":8544,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3273,"record_offset":17324,"parent_record_offset":17296,"ancestor_record_offset":8544,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3273,"record_offset":17360,"parent_record_offset":17324,"ancestor_record_offset":8544,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3273,"record_offset":17296,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3273,"record_offset":17156,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3273,"record_offset":17888,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3273,"record_offset":17240,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3273,"record_offset":17128,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3273,"record_offset":17656,"parent_record_offset":17628,"ancestor_record_offset":8544,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":16908,"parent_record_offset":16880,"ancestor_record_offset":8544,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":16936,"parent_record_offset":16908,"ancestor_record_offset":8544,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":15844,"parent_record_offset":15804,"ancestor_record_offset":8544,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":17628,"parent_record_offset":17460,"ancestor_record_offset":8544,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":15804,"parent_record_offset":15776,"ancestor_record_offset":8544,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3273,"record_offset":16968,"parent_record_offset":16936,"ancestor_record_offset":8544,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17568,"parent_record_offset":17484,"ancestor_record_offset":8544,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":8544,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17980,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":8544,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17020,"parent_record_offset":16996,"ancestor_record_offset":8544,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":16996,"parent_record_offset":16936,"ancestor_record_offset":8544,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":8544,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17044,"parent_record_offset":17020,"ancestor_record_offset":8544,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17100,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":8544,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3273,"record_offset":17460,"parent_record_offset":17044,"ancestor_record_offset":8544,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":3273,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":8544,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":3273,"record_offset":15776,"parent_record_offset":15748,"ancestor_record_offset":8544,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":9216,"parent_record_offset":9128,"ancestor_record_offset":8544,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":12740,"parent_record_offset":12600,"ancestor_record_offset":8544,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":8544,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":8544,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":8544,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3273,"record_offset":12600,"parent_record_offset":12560,"ancestor_record_offset":8544,"inlinee":113613,"source_line":408,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3273,"record_offset":18588,"parent_record_offset":18556,"ancestor_record_offset":8544,"inlinee":113618,"source_line":473,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":3273,"record_offset":15720,"parent_record_offset":14996,"ancestor_record_offset":8544,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":3273,"record_offset":14996,"parent_record_offset":14956,"ancestor_record_offset":8544,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":3273,"record_offset":15748,"parent_record_offset":14996,"ancestor_record_offset":8544,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3273,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":8544,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3273,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":8544,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3273,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":8544,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3273,"record_offset":15520,"parent_record_offset":15120,"ancestor_record_offset":8544,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3273,"record_offset":15048,"parent_record_offset":14996,"ancestor_record_offset":8544,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3273,"record_offset":15212,"parent_record_offset":15184,"ancestor_record_offset":8544,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3273,"record_offset":15244,"parent_record_offset":15212,"ancestor_record_offset":8544,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3273,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":8544,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3273,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":8544,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBCD
// 首条位置: {"module":3273,"inlinee":113613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3273,"record_offset":12708,"parent_record_offset":12600,"ancestor_record_offset":12600,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":12740,"parent_record_offset":12600,"ancestor_record_offset":12600,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":12600,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":12796,"parent_record_offset":12764,"ancestor_record_offset":12600,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":12824,"parent_record_offset":12764,"ancestor_record_offset":12600,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":12600,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":12600,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":12852,"parent_record_offset":12764,"ancestor_record_offset":12600,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":12600,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":12876,"parent_record_offset":12852,"ancestor_record_offset":12600,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155CC
// 首条位置: {"module":1988,"inlinee":87500}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1988,"record_offset":808,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ExtensionProperties::extension_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1988,"record_offset":752,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":39991,"source_line":991,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1988,"record_offset":1144,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":888,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":712,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1988,"record_offset":1064,"parent_record_offset":1008,"ancestor_record_offset":712,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1988,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":712,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":712,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":712,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1988,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":712,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::prelude::wrap_c_str_slice_until_nul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/prelude.rs
//     evidence={"module":1988,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":712,"inlinee":39993,"source_line":67,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1988,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":712,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1988,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":712,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01988.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152CE
// 首条位置: {"module":1935,"inlinee":86734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1935,"record_offset":2080,"parent_record_offset":1916,"ancestor_record_offset":1916,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":1935,"record_offset":1964,"parent_record_offset":1916,"ancestor_record_offset":1916,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":2752,"parent_record_offset":1916,"ancestor_record_offset":1916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1916,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1916,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1916,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":2572,"parent_record_offset":2372,"ancestor_record_offset":1916,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1935,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1916,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2308,"parent_record_offset":2276,"ancestor_record_offset":1916,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1916,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2372,"parent_record_offset":2340,"ancestor_record_offset":1916,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2340,"parent_record_offset":2308,"ancestor_record_offset":1916,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1916,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1935,"record_offset":2412,"parent_record_offset":2372,"ancestor_record_offset":1916,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1935,"record_offset":2240,"parent_record_offset":2208,"ancestor_record_offset":1916,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1935,"record_offset":2208,"parent_record_offset":2176,"ancestor_record_offset":1916,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1935,"record_offset":2144,"parent_record_offset":2112,"ancestor_record_offset":1916,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1935,"record_offset":2176,"parent_record_offset":2144,"ancestor_record_offset":1916,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1935,"record_offset":2112,"parent_record_offset":2080,"ancestor_record_offset":1916,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1935,"record_offset":2276,"parent_record_offset":2240,"ancestor_record_offset":1916,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1935,"record_offset":2440,"parent_record_offset":2412,"ancestor_record_offset":1916,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1935,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":1916,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":1935,"record_offset":2004,"parent_record_offset":1964,"ancestor_record_offset":1916,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1935,"record_offset":2036,"parent_record_offset":2004,"ancestor_record_offset":1916,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1935,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":1916,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1935,"record_offset":2524,"parent_record_offset":2496,"ancestor_record_offset":1916,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1935,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":1916,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1935,"record_offset":2672,"parent_record_offset":2240,"ancestor_record_offset":1916,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1916,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBD2
// 首条位置: {"module":3273,"inlinee":113618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3273,"record_offset":18708,"parent_record_offset":18588,"ancestor_record_offset":18588,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3273,"record_offset":18916,"parent_record_offset":18588,"ancestor_record_offset":18588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":18740,"parent_record_offset":18588,"ancestor_record_offset":18588,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":18624,"parent_record_offset":18588,"ancestor_record_offset":18588,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":18796,"parent_record_offset":18764,"ancestor_record_offset":18588,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3273,"record_offset":18824,"parent_record_offset":18764,"ancestor_record_offset":18588,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":18648,"parent_record_offset":18624,"ancestor_record_offset":18588,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":18672,"parent_record_offset":18648,"ancestor_record_offset":18588,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":18852,"parent_record_offset":18764,"ancestor_record_offset":18588,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3273,"record_offset":18764,"parent_record_offset":18740,"ancestor_record_offset":18588,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3273,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":18588,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::find_vendor_specific_device_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 498
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":704,"record_offset":18868,"rva":12590720,"va":5381299840}
// 编译实例 RVAs: 0xc01e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1344362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e86a831400","size":5,"soft_fail":false,"target_va":5382644528,"va":5381300161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6218835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e8ad1ba1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381300334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5784019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e82dbea7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381300046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30181833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e8c989cc01","size":5,"soft_fail":false,"target_va":5411482272,"va":5381300434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30438898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e8f275d001","size":5,"soft_fail":false,"target_va":5411739344,"va":5381300441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi::gpu::PhysicalGpu::pci_identifiers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi","version":"0.1.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13301880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e878f8ca00","size":5,"soft_fail":false,"target_va":5394601904,"va":5381300019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] nvapi_hi::gpu::Gpu::enumerate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"nvapi-hi","version":"0.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13300864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381299840,"raw_hex":"e880f4ca00","size":5,"soft_fail":false,"target_va":5394600768,"va":5381299899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":704,"record_offset":19084,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":704,"record_offset":19140,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":704,"record_offset":19340,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":704,"record_offset":20472,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":19840,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":19052,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":704,"record_offset":20500,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":704,"record_offset":19372,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":704,"record_offset":19256,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":18868,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":704,"record_offset":20072,"parent_record_offset":19984,"ancestor_record_offset":18868,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":20012,"parent_record_offset":19984,"ancestor_record_offset":18868,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":18868,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":704,"record_offset":19960,"parent_record_offset":19936,"ancestor_record_offset":18868,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":704,"record_offset":19864,"parent_record_offset":19840,"ancestor_record_offset":18868,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":704,"record_offset":19912,"parent_record_offset":19888,"ancestor_record_offset":18868,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":704,"record_offset":20040,"parent_record_offset":20012,"ancestor_record_offset":18868,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":704,"record_offset":19428,"parent_record_offset":19396,"ancestor_record_offset":18868,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":704,"record_offset":19456,"parent_record_offset":19396,"ancestor_record_offset":18868,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":19888,"parent_record_offset":19864,"ancestor_record_offset":18868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":704,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":18868,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":19196,"parent_record_offset":19140,"ancestor_record_offset":18868,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":704,"record_offset":19168,"parent_record_offset":19140,"ancestor_record_offset":18868,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":704,"record_offset":19220,"parent_record_offset":19196,"ancestor_record_offset":18868,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":19280,"parent_record_offset":19256,"ancestor_record_offset":18868,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":704,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":18868,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":704,"record_offset":19484,"parent_record_offset":19396,"ancestor_record_offset":18868,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":704,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":18868,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":704,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":18868,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::string_from_c_array
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BE3
// 首条位置: {"module":3273,"inlinee":39907}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3273,"record_offset":10620,"parent_record_offset":10572,"ancestor_record_offset":10572,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":10572,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":10572,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3273,"record_offset":11060,"parent_record_offset":10884,"ancestor_record_offset":10572,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":10572,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10884,"parent_record_offset":10852,"ancestor_record_offset":10572,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":10572,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3273,"record_offset":10920,"parent_record_offset":10884,"ancestor_record_offset":10572,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10760,"parent_record_offset":10732,"ancestor_record_offset":10572,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10732,"parent_record_offset":10704,"ancestor_record_offset":10572,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10676,"parent_record_offset":10648,"ancestor_record_offset":10572,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":10572,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3273,"record_offset":10648,"parent_record_offset":10620,"ancestor_record_offset":10572,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3273,"record_offset":10796,"parent_record_offset":10760,"ancestor_record_offset":10572,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3273,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10572,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3273,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10572,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":10572,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":10572,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3273,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":10572,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3273,"record_offset":11160,"parent_record_offset":10760,"ancestor_record_offset":10572,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support
// 记录形式: compiled_procedure; instances=1
// 源码行: 540
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":685,"record_offset":256,"rva":12439440,"va":5381148560}
// 编译实例 RVAs: 0xbdcf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13502617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381148560,"raw_hex":"e89908ce00","size":5,"soft_fail":false,"target_va":5394651248,"va":5381148626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":685,"record_offset":412,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":685,"record_offset":492,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":685,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":256,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":685,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":256,"inlinee":11705,"source_line":251,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":685,"record_offset":568,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":11928,"source_line":233,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":685,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":685,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":256,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":685,"record_offset":460,"parent_record_offset":412,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00685.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 542
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBC1
// 首条位置: {"module":3273,"inlinee":113601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":6116,"parent_record_offset":6068,"ancestor_record_offset":6068,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":3273,"record_offset":6428,"parent_record_offset":6364,"ancestor_record_offset":6068,"inlinee":39678,"source_line":12,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3273,"record_offset":6400,"parent_record_offset":6364,"ancestor_record_offset":6068,"inlinee":39634,"source_line":32,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":6460,"parent_record_offset":6428,"ancestor_record_offset":6068,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":6068,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3273,"record_offset":6148,"parent_record_offset":6116,"ancestor_record_offset":6068,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3273,"record_offset":6296,"parent_record_offset":6180,"ancestor_record_offset":6068,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":6332,"parent_record_offset":6296,"ancestor_record_offset":6068,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3273,"record_offset":6180,"parent_record_offset":6148,"ancestor_record_offset":6068,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3273,"record_offset":6260,"parent_record_offset":6220,"ancestor_record_offset":6068,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":6520,"parent_record_offset":6364,"ancestor_record_offset":6068,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3273,"record_offset":6220,"parent_record_offset":6180,"ancestor_record_offset":6068,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3273,"record_offset":6364,"parent_record_offset":6332,"ancestor_record_offset":6068,"inlinee":113603,"source_line":548,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 548
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBC3
// 首条位置: {"module":3273,"inlinee":113603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":3273,"record_offset":6428,"parent_record_offset":6364,"ancestor_record_offset":6364,"inlinee":39678,"source_line":12,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":3273,"record_offset":6400,"parent_record_offset":6364,"ancestor_record_offset":6364,"inlinee":39634,"source_line":32,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":6520,"parent_record_offset":6364,"ancestor_record_offset":6364,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":6460,"parent_record_offset":6428,"ancestor_record_offset":6364,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3273,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":6364,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::parse_driver_version
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BE4
// 首条位置: {"module":3273,"inlinee":39908}
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
// 完整结构及原始类型记录: physical_device.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::DeviceDriverInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::physical_device::DeviceDriverInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::physical_device::DeviceDriverInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::DeviceDriverVersion
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::PhysicalDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::slice::iter::Iter", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["alloc::string::String", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::string::String", "alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::string::String", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>", "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>,rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>", "rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >", "constructor": "core::result::impl$25::from_iter::closure_env$0", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "ash::vk::enums::Result", "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::iter::adapters::try_process", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>", "rhapsody::raw::physical_device::PhysicalDevice", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >", "core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >", "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$>", "constructor": "core::result::Result", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >", "constructor": "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "rhapsody::raw::physical_device::PhysicalDevice", "alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >", "rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$5::try_fold<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> > >", "constructor": "alloc::vec::into_iter::impl$5::try_fold", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global", "alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>", "core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result>", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "ash::vk::enums::Result"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>", "constructor": "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >", "enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > >", "constructor": "core::iter::adapters::impl$0::try_fold::closure_env$0", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >", "tuple$<>", "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>", "enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >", "constructor": "core::iter::adapters::map::map_try_fold::closure_env$0", "arguments": ["ash::vk::definitions::PhysicalDevice", "enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >", "rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1", "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >", "constructor": "alloc::vec::into_iter::impl$5::try_fold", "arguments": ["ash::vk::definitions::PhysicalDevice", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >", "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::QueueFamily
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::option::impl$46::fmt<rhapsody::raw::physical_device::QueueFamily>", "constructor": "core::option::impl$46::fmt", "arguments": ["rhapsody::raw::physical_device::QueueFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::QueueFamily,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<ash::vk::definitions::QueueFamilyProperties,alloc::alloc::Global> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::physical_device::QueueFamily", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<ash::vk::definitions::QueueFamilyProperties,alloc::alloc::Global> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<rhapsody::raw::physical_device::QueueFamily>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["rhapsody::raw::physical_device::QueueFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: physical_device.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: physical_device.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::DeviceDriverVersion::is_amd
// 信息 ID: 3554179da22e5761ebdfd52a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59d62a -> 0xc001e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::DeviceDriverVersion::is_apple
// 信息 ID: 99755ed614cb5f0b25813633
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x595cc0 -> 0xc001f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5969f0 -> 0xc001f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
// 信息 ID: 73a07d7996812b08945d004d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598b85 -> 0xc001c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
// 信息 ID: ed2627ed9939ee1d21a2e855
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x4f7f65 -> 0xc006a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x4f83d4 -> 0xc006a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x4f8669 -> 0xc006a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
// 信息 ID: e3835f757f15ce5bbbc67b38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf3c22 -> 0xc00530 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf3e4e -> 0xc00530 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=4836; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
// 信息 ID: d5d6223731310170a9742496
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x4f805e -> 0xc00890 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x4f8125 -> 0xc00890 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x4f8400 -> 0xc00890 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x4f8674 -> 0xc00890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
// 信息 ID: 1a89befc6d6f0ec3e4b84480
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=5364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::name
// 信息 ID: 758f1cf557b2ff11acbc09fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$20::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > > @ 0x48eae0 -> 0xc00400 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::create_device @ 0xbdb9de -> 0xc00400 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > > @ 0x18b348f -> 0xc00400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1d1bf -> 0xc00400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::performance_info
// 信息 ID: 7ef3c8115baaf12e03d06e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
// 信息 ID: 03ae8ac1e450e19a157ff687
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget @ 0xbf8f91 -> 0xc01620 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices
// 信息 ID: 3ed4e6aa2061defa3cf206cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > @ 0x340e0b -> 0xc01e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
// 信息 ID: 34e7fda4740b209afeaaad9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0 <- rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; module=1988, inline_record=1672; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; module=1988, inline_record=2140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1
// 信息 ID: dbfa4c9c659ee81844266a95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=8544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
// 信息 ID: 0d82853ff4a4cef9f02a0bc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=12600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$2
// 信息 ID: 6f9342c1ce67daefa5fed2b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; module=1935, inline_record=1916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$3
// 信息 ID: a647d8829cbcd618123ef2c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=18588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$4
// 信息 ID: badedd55a5e4d59e82192077
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$4 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ash::vk::definitions::ExtensionProperties> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ash::vk::definitions::ExtensionProperties> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$4>; module=1988, inline_record=712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 信息 ID: 995607e5685e99e5aa6ca567
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::physical_device::enumerate_vk_data -> ["u32", "ash::vk::definitions::PhysicalDevice", "rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::physical_device::enumerate_physical_devices @ 0xc01e11 -> 0x18cd4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::find_vendor_specific_device_handle
// 信息 ID: 4e2518d55615cf56dfb973c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > > @ 0x2137701 -> 0xc01e80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$13::fmt
// 信息 ID: 9749a349d09e28386f403235
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$16::fmt
// 信息 ID: cafa6e407769ee81460a4af6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<rhapsody::raw::physical_device::QueueFamily>; module=1988, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$17::clone
// 信息 ID: 1f56ab5648de4b0728c33906
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=49924; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>; module=936, inline_record=1360; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=4444; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1936, inline_record=8592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$18::fmt
// 信息 ID: 0aac2d085c0ba652261a6ce9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$19::clone
// 信息 ID: 9bf428ffbbd70b99b7f109b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=50056; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>; module=936, inline_record=1456; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=4576; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1936, inline_record=8724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$1::fmt
// 信息 ID: 2d554c12761d449cf9f89693
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$21::fmt
// 信息 ID: c9799a16fef63709b292f79b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$3::fmt
// 信息 ID: 1132129ef45c0894690d2a66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x598b52 -> 0xc00200 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::render_config::impl$7::from @ 0x968cb3 -> 0xc00200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::physical_device::impl$1::fmt @ 0xbffef5 -> 0xc00200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
// 信息 ID: 084301df8c1664346e346dd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes; module=704, inline_record=8032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
// 信息 ID: e79d0d621bdd5b8676716770
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::device_local_memory_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes; module=704, inline_record=7136; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=5228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
// 信息 ID: a07be5df472694378f388cb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes; module=704, inline_record=9464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
// 信息 ID: 8f7605e5d92ae5f8451800e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::host_memory_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=1935, inline_record=5548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$5::fmt
// 信息 ID: 8aa0d176c2fb07aca801d9ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$54::fmt<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>; module=687, inline_record=5536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support
// 信息 ID: fb510ace7af01a010baebf2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > @ 0x340e85 -> 0xbdcf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0
// 信息 ID: 9922da0012f76c27580d3aef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$6::with_presentation_support", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>,rhapsody::raw::physical_device::PhysicalDevice>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0 <- alloc::vec::into_iter::impl$5::try_fold<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> > >; module=3273, inline_record=6068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
// 信息 ID: e79e52a737359ad26f472f1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$6::with_presentation_support", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> >,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>,never$> > >; module=3273, inline_record=6364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::parse_driver_version
// 信息 ID: 74d0f80d1de0c04205b029e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=11912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::string_from_c_array
// 信息 ID: 15f1d1b0eb47839c937865a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=10572; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<ash::vk::definitions::PhysicalDevice,enum2$<core::result::Result<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > >,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > >,tuple$<> > > >; module=3273, inline_record=11244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
