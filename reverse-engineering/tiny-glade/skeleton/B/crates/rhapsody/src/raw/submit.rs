// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/submit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 25f429803a273e218b30beb86d490522a30f5212f7c9a4fba1c5d2ebcd7e8aaf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: submit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 690, 699, 702, 703, 707, 1935, 1936, 1953, 1970, 1987
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.18.1"}]
// use core; // namespace candidate
//   package identities: []
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use spin_sleep; // namespace candidate
//   package identities: [{"name":"spin_sleep","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A53
// 首条位置: {"module":699,"inlinee":39507}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A51
// 首条位置: {"module":699,"inlinee":39505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::submit::impl$3::default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":699,"record_offset":19504,"parent_record_offset":19480,"ancestor_record_offset":19480,"inlinee":39507,"source_line":10,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::command_buffer
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A54
// 首条位置: {"module":699,"inlinee":39508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":699,"record_offset":19576,"parent_record_offset":19536,"ancestor_record_offset":19536,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":699,"record_offset":19640,"parent_record_offset":19616,"ancestor_record_offset":19536,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":699,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19536,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":699,"record_offset":19664,"parent_record_offset":19640,"ancestor_record_offset":19536,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":699,"record_offset":19616,"parent_record_offset":19576,"ancestor_record_offset":19536,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":699,"record_offset":19728,"parent_record_offset":19576,"ancestor_record_offset":19536,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00699.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::command_buffers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C0E
// 首条位置: {"module":707,"inlinee":39950}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20200,"parent_record_offset":20164,"ancestor_record_offset":20164,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":20164,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":20252,"parent_record_offset":20224,"ancestor_record_offset":20164,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::wait_semaphore
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C0F
// 首条位置: {"module":707,"inlinee":39951}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20332,"parent_record_offset":20292,"ancestor_record_offset":20292,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":20292,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":20292,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":20292,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20368,"parent_record_offset":20332,"ancestor_record_offset":20292,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":20480,"parent_record_offset":20332,"ancestor_record_offset":20292,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmitInfo::signal_semaphore
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C10
// 首条位置: {"module":707,"inlinee":39952}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20740,"parent_record_offset":20700,"ancestor_record_offset":20700,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20804,"parent_record_offset":20780,"ancestor_record_offset":20700,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20852,"parent_record_offset":20828,"ancestor_record_offset":20700,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":20828,"parent_record_offset":20804,"ancestor_record_offset":20700,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20780,"parent_record_offset":20740,"ancestor_record_offset":20700,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":20892,"parent_record_offset":20740,"ancestor_record_offset":20700,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::carve_off_front
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152DD
// 首条位置: {"module":1935,"inlinee":86749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16736,"parent_record_offset":16700,"ancestor_record_offset":16700,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1935,"record_offset":16860,"parent_record_offset":16736,"ancestor_record_offset":16700,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1935,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":16700,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16772,"parent_record_offset":16736,"ancestor_record_offset":16700,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16800,"parent_record_offset":16772,"ancestor_record_offset":16700,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9ACB
// 首条位置: {"module":703,"inlinee":39627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":40692,"parent_record_offset":40664,"ancestor_record_offset":40664,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":40716,"parent_record_offset":40692,"ancestor_record_offset":40664,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$9::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AA1
// 首条位置: {"module":702,"record_offset":3964,"rva":12507024,"va":5381216144}
// 编译实例 RVAs: 0xbed790, 0xc08de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1678085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381216144,"raw_hex":"e8059b1900","size":5,"soft_fail":false,"target_va":5382894288,"va":5381216198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1676198","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381216144,"raw_hex":"e9a6931900","size":5,"soft_fail":false,"target_va":5382892432,"va":5381216229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$17::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AC7
// 首条位置: {"module":703,"inlinee":39623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":28108,"parent_record_offset":28080,"ancestor_record_offset":28080,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C12
// 首条位置: {"module":707,"inlinee":39954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23280,"parent_record_offset":23152,"ancestor_record_offset":23152,"inlinee":39955,"source_line":3143,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":24724,"parent_record_offset":23152,"ancestor_record_offset":23152,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":707,"record_offset":23196,"parent_record_offset":23152,"ancestor_record_offset":23152,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::update_pending_submission_timestamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":24752,"parent_record_offset":23152,"ancestor_record_offset":23152,"inlinee":39327,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":23152,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":23152,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":23152,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24488,"parent_record_offset":24300,"ancestor_record_offset":23152,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24332,"parent_record_offset":24300,"ancestor_record_offset":23152,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":23152,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24420,"parent_record_offset":24388,"ancestor_record_offset":23152,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24512,"parent_record_offset":24488,"ancestor_record_offset":23152,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24444,"parent_record_offset":24420,"ancestor_record_offset":23152,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24560,"parent_record_offset":24536,"ancestor_record_offset":23152,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24536,"parent_record_offset":24512,"ancestor_record_offset":23152,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":707,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":23152,"inlinee":14050,"source_line":1502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":707,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":23152,"inlinee":10145,"source_line":118,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":707,"record_offset":23424,"parent_record_offset":23396,"ancestor_record_offset":23152,"inlinee":10146,"source_line":101,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":24224,"parent_record_offset":24064,"ancestor_record_offset":23152,"inlinee":18118,"source_line":39,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":24016,"parent_record_offset":23336,"ancestor_record_offset":23152,"inlinee":18108,"source_line":142,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":23368,"parent_record_offset":23336,"ancestor_record_offset":23152,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23396,"parent_record_offset":23368,"ancestor_record_offset":23152,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23512,"parent_record_offset":23484,"ancestor_record_offset":23152,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23308,"parent_record_offset":23280,"ancestor_record_offset":23152,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":24640,"parent_record_offset":24224,"ancestor_record_offset":23152,"inlinee":14049,"source_line":3265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23336,"parent_record_offset":23308,"ancestor_record_offset":23152,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23456,"parent_record_offset":23424,"ancestor_record_offset":23152,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":707,"record_offset":23484,"parent_record_offset":23456,"ancestor_record_offset":23152,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":24608,"parent_record_offset":24252,"ancestor_record_offset":23152,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":24188,"parent_record_offset":24164,"ancestor_record_offset":23152,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":23152,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":24164,"parent_record_offset":24140,"ancestor_record_offset":23152,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":23152,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":24140,"parent_record_offset":24064,"ancestor_record_offset":23152,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":23744,"parent_record_offset":23708,"ancestor_record_offset":23152,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":707,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":23152,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":23152,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":23152,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":23152,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23772,"parent_record_offset":23708,"ancestor_record_offset":23152,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":23568,"parent_record_offset":23540,"ancestor_record_offset":23152,"inlinee":39957,"source_line":110,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C15
// 首条位置: {"module":707,"inlinee":39957}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":23568,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":24608,"parent_record_offset":24252,"ancestor_record_offset":24252,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":24252,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":23568,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24488,"parent_record_offset":24300,"ancestor_record_offset":24252,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24332,"parent_record_offset":24300,"ancestor_record_offset":24252,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":24252,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24420,"parent_record_offset":24388,"ancestor_record_offset":24252,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24512,"parent_record_offset":24488,"ancestor_record_offset":24252,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24444,"parent_record_offset":24420,"ancestor_record_offset":24252,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24560,"parent_record_offset":24536,"ancestor_record_offset":24252,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":24536,"parent_record_offset":24512,"ancestor_record_offset":24252,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1970,"record_offset":2036,"rva":25965312,"va":5394674432}
// 编译实例 RVAs: 0x18c3300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16866339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8235c0101","size":5,"soft_fail":false,"target_va":5411541584,"va":5394675240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e894fbffff","size":5,"soft_fail":false,"target_va":5394673360,"va":5394674487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16868791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8b7650101","size":5,"soft_fail":false,"target_va":5411543360,"va":5394674564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13289466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8063835ff","size":5,"soft_fail":false,"target_va":5381385536,"va":5394674997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13284280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8484c35ff","size":5,"soft_fail":false,"target_va":5381390640,"va":5394674915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13288673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e81f3b35ff","size":5,"soft_fail":false,"target_va":5381386480,"va":5394675148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13289766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8da3635ff","size":5,"soft_fail":false,"target_va":5381384704,"va":5394674465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::update_pending_submissions
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13445581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e833d632ff","size":5,"soft_fail":false,"target_va":5381229456,"va":5394675032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17062621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8dd5a0401","size":5,"soft_fail":false,"target_va":5411737808,"va":5394675182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e863f7ffff","size":5,"soft_fail":false,"target_va":5394673152,"va":5394675352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16861871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394674432,"raw_hex":"e8af4a0101","size":5,"soft_fail":false,"target_va":5411537104,"va":5394675228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1970,"record_offset":4616,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1970,"record_offset":5028,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":4876,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":3648,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":2216,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":2672,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1970,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":2036,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1970,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":2036,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1970,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":2036,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1970,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":2036,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1970,"record_offset":4672,"parent_record_offset":4640,"ancestor_record_offset":2036,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1970,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":2036,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":2036,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":2036,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":2036,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":3916,"parent_record_offset":3888,"ancestor_record_offset":2036,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":2036,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":4112,"parent_record_offset":3940,"ancestor_record_offset":2036,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1970,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":2036,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1970,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2036,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1970,"record_offset":2372,"parent_record_offset":2340,"ancestor_record_offset":2036,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1970,"record_offset":3784,"parent_record_offset":3744,"ancestor_record_offset":2036,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1970,"record_offset":2620,"parent_record_offset":2592,"ancestor_record_offset":2036,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1970,"record_offset":4572,"parent_record_offset":3744,"ancestor_record_offset":2036,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1970,"record_offset":4176,"parent_record_offset":3888,"ancestor_record_offset":2036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1970,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2036,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1970,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":2036,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1970,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2036,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1970,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2036,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1970,"record_offset":3268,"parent_record_offset":3240,"ancestor_record_offset":2036,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1970,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2036,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2036,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1970,"record_offset":4812,"parent_record_offset":4640,"ancestor_record_offset":2036,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":2036,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":2036,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":2988,"parent_record_offset":2872,"ancestor_record_offset":2036,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1970,"record_offset":4940,"parent_record_offset":4876,"ancestor_record_offset":2036,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1970,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":2036,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1970,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":2036,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1970,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":2036,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1970,"record_offset":2808,"parent_record_offset":2772,"ancestor_record_offset":2036,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1970,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":2036,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1970,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":2036,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1970,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":2036,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1970,"record_offset":3312,"parent_record_offset":2772,"ancestor_record_offset":2036,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":2036,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":2036,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2472,"parent_record_offset":2440,"ancestor_record_offset":2036,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2568,"parent_record_offset":2472,"ancestor_record_offset":2036,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":2036,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2252,"parent_record_offset":2216,"ancestor_record_offset":2036,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1970,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2036,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1970,"record_offset":2872,"parent_record_offset":2840,"ancestor_record_offset":2036,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1970,"record_offset":3156,"parent_record_offset":2872,"ancestor_record_offset":2036,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1970,"record_offset":3128,"parent_record_offset":2872,"ancestor_record_offset":2036,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1970,"record_offset":2932,"parent_record_offset":2872,"ancestor_record_offset":2036,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":3680,"parent_record_offset":3648,"ancestor_record_offset":2036,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":5092,"parent_record_offset":5060,"ancestor_record_offset":2036,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":5592,"parent_record_offset":5332,"ancestor_record_offset":2036,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1970,"record_offset":3812,"parent_record_offset":3784,"ancestor_record_offset":2036,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":3888,"parent_record_offset":3744,"ancestor_record_offset":2036,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1970,"record_offset":4148,"parent_record_offset":3888,"ancestor_record_offset":2036,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1970,"record_offset":3840,"parent_record_offset":3812,"ancestor_record_offset":2036,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1970,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":2036,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1970,"record_offset":3712,"parent_record_offset":3680,"ancestor_record_offset":2036,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01970.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x998B
// 首条位置: {"module":690,"inlinee":39307}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102596,"parent_record_offset":102564,"ancestor_record_offset":102564,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102680,"parent_record_offset":102564,"ancestor_record_offset":102564,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102644,"parent_record_offset":102620,"ancestor_record_offset":102564,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102620,"parent_record_offset":102596,"ancestor_record_offset":102564,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":690,"record_offset":102804,"parent_record_offset":102780,"ancestor_record_offset":102564,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":690,"record_offset":102724,"parent_record_offset":102680,"ancestor_record_offset":102564,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102780,"parent_record_offset":102756,"ancestor_record_offset":102564,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102756,"parent_record_offset":102680,"ancestor_record_offset":102564,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::queue
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B4A
// 首条位置: {"module":703,"inlinee":39754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":124248,"parent_record_offset":124208,"ancestor_record_offset":124208,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":124276,"parent_record_offset":124248,"ancestor_record_offset":124208,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":124304,"parent_record_offset":124276,"ancestor_record_offset":124208,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::SubmissionEngine::queue_mut
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C11
// 首条位置: {"module":707,"inlinee":39953}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":23052,"parent_record_offset":23012,"ancestor_record_offset":23012,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":23080,"parent_record_offset":23052,"ancestor_record_offset":23012,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":23012,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::queue_family_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":121208,"rva":12568400,"va":5381277520}
// 编译实例 RVAs: 0xbfc750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30264211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277520,"raw_hex":"e893cbcd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381277560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":121352,"parent_record_offset":121208,"ancestor_record_offset":121208,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":16508,"rva":12602912,"va":5381312032}
// 编译实例 RVAs: 0xc04e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$T$GT$$GT$::extend
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13361932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e80ce3cb00","size":5,"soft_fail":false,"target_va":5394676608,"va":5381314671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12345873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ef9d43ff","size":5,"soft_fail":false,"target_va":5368976000,"va":5381321868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7689439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e821ab8aff","size":5,"soft_fail":false,"target_va":5373629488,"va":5381318922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6455700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e86c7e9dff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381322639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6425877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ebf29dff","size":5,"soft_fail":false,"target_va":5374895648,"va":5381321520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6232751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e851e5a0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381314234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=29
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6234372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8fcdea0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381315871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6232659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ade5a0ff","size":5,"soft_fail":false,"target_va":5375081520,"va":5381314174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5802814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8c274a7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381318841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4426360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e88875bcff","size":5,"soft_fail":false,"target_va":5376895600,"va":5381321955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-698933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8cb55f5ff","size":5,"soft_fail":false,"target_va":5380622000,"va":5381320928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-170543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8d165fdff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381323050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf860
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-155005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e883a2fdff","size":5,"soft_fail":false,"target_va":5381159008,"va":5381314008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be14b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-149565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8c3b7fdff","size":5,"soft_fail":false,"target_va":5381166256,"va":5381315816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be1ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e883adfdff","size":5,"soft_fail":false,"target_va":5381167792,"va":5381319976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be1fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8bcb1fdff","size":5,"soft_fail":false,"target_va":5381169088,"va":5381320191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e83596feff","size":5,"soft_fail":false,"target_va":5381225824,"va":5381318438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf0720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e807b6feff","size":5,"soft_fail":false,"target_va":5381228320,"va":5381312788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf0860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8cfa5feff","size":5,"soft_fail":false,"target_va":5381228640,"va":5381317260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13302676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e894fbca00","size":5,"soft_fail":false,"target_va":5394616144,"va":5381313463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b6db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13309390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ce15cb00","size":5,"soft_fail":false,"target_va":5394623920,"va":5381314525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b7b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13311460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8e41dcb00","size":5,"soft_fail":false,"target_va":5394627440,"va":5381315975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b7e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13312057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e83920cb00","size":5,"soft_fail":false,"target_va":5394628208,"va":5381316146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13314422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e87629cb00","size":5,"soft_fail":false,"target_va":5394630496,"va":5381316069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13316134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e82630cb00","size":5,"soft_fail":false,"target_va":5394632480,"va":5381316341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30534284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e88cead101","size":5,"soft_fail":false,"target_va":5411847168,"va":5381312879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30534116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8e4e9d101","size":5,"soft_fail":false,"target_va":5411847184,"va":5381313063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-700329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85750f5ff","size":5,"soft_fail":false,"target_va":5380615456,"va":5381315780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-6109059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e87dc8a2ff","size":5,"soft_fail":false,"target_va":5375203984,"va":5381313038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30222459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e87b28cd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381318499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t515494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8a6dd0700","size":5,"soft_fail":false,"target_va":5381832720,"va":5381317221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1328478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85e451400","size":5,"soft_fail":false,"target_va":5382644656,"va":5381316173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30223293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8bd2bcd01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381318286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30223255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8972bcd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381318516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30165075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85348cc01","size":5,"soft_fail":false,"target_va":5411481740,"va":5381316660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30225324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ac33cd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381316735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30223595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8eb2ccd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381318696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30223630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e80e2dcd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381318269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e84eb5ffff","size":5,"soft_fail":false,"target_va":5381303760,"va":5381322877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30224241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8712fcd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381318634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e820e0ffff","size":5,"soft_fail":false,"target_va":5381303984,"va":5381312139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30231132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85c4acd01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381312223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13353594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e87ac2cb00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381312769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30174369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8a16ccc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381317994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30176690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8b275cc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381318105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30171950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e82e63cc01","size":5,"soft_fail":false,"target_va":5411488944,"va":5381316989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30173530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85a69cc01","size":5,"soft_fail":false,"target_va":5411491664,"va":5381318129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t510741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e815cb0700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381312854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t506894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e80ebc0700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381317117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t72825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8791c0100","size":5,"soft_fail":false,"target_va":5381385536,"va":5381312706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t78046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8de300100","size":5,"soft_fail":false,"target_va":5381390640,"va":5381312589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t69101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8ed0d0100","size":5,"soft_fail":false,"target_va":5381386480,"va":5381317374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t72594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8921b0100","size":5,"soft_fail":false,"target_va":5381384704,"va":5381312105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-52405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e84b33ffff","size":5,"soft_fail":false,"target_va":5381260512,"va":5381312912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-87655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e899a9feff","size":5,"soft_fail":false,"target_va":5381229760,"va":5381317410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13346644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e854a7cb00","size":5,"soft_fail":false,"target_va":5394664048,"va":5381317399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13360349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8dddccb00","size":5,"soft_fail":false,"target_va":5394674432,"va":5381314078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::exit
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1427969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e801ca1500","size":5,"soft_fail":false,"target_va":5382746448,"va":5381318474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30425174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e85640d001","size":5,"soft_fail":false,"target_va":5411737808,"va":5381312629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13351355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8bbb9cb00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381321792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30218448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381312032,"raw_hex":"e8d018cd01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381318651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*136(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381312032,"raw_hex":"ff9088000000","size":6,"soft_fail":false,"va":5381317099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":25264,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":25904,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":25200,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":707,"record_offset":25728,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":707,"record_offset":35404,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":21368,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":707,"record_offset":35436,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":707,"record_offset":35292,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":21968,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":27304,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":25072,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":32200,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":18849,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":32232,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":26264,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":31072,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":30952,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":707,"record_offset":21924,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":24920,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":19364,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":707,"record_offset":35320,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":18324,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":16900,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":17348,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::queue_submit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":19332,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39352,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::vk_queue_submit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":29528,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39353,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::Queue::with_raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":707,"record_offset":29560,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39958,"source_line":70,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":23012,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39953,"source_line":151,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":23152,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39954,"source_line":107,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":25876,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39627,"source_line":76,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::executor::impl$7::submit_pending_cbs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/executor.rs
//     evidence={"module":707,"record_offset":19520,"parent_record_offset":16508,"ancestor_record_offset":16508,"inlinee":39949,"source_line":560,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":22444,"parent_record_offset":22420,"ancestor_record_offset":16508,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":22468,"parent_record_offset":22444,"ancestor_record_offset":16508,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":21860,"parent_record_offset":21836,"ancestor_record_offset":16508,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":22420,"parent_record_offset":22248,"ancestor_record_offset":16508,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":21836,"parent_record_offset":21624,"ancestor_record_offset":16508,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":21752,"parent_record_offset":21624,"ancestor_record_offset":16508,"inlinee":7420,"source_line":299,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":16508,"inlinee":7421,"source_line":135,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":707,"record_offset":33724,"parent_record_offset":33664,"ancestor_record_offset":16508,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":16508,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":16508,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":16508,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":25612,"parent_record_offset":25264,"ancestor_record_offset":16508,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24488,"parent_record_offset":24300,"ancestor_record_offset":16508,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24332,"parent_record_offset":24300,"ancestor_record_offset":16508,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":707,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":16508,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21156,"parent_record_offset":21112,"ancestor_record_offset":16508,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":19760,"parent_record_offset":19736,"ancestor_record_offset":16508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21524,"parent_record_offset":21488,"ancestor_record_offset":16508,"inlinee":7416,"source_line":393,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":22188,"parent_record_offset":22160,"ancestor_record_offset":16508,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":16508,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21668,"parent_record_offset":21624,"ancestor_record_offset":16508,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":26616,"parent_record_offset":26592,"ancestor_record_offset":16508,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":16508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":19784,"parent_record_offset":19760,"ancestor_record_offset":16508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21596,"parent_record_offset":21560,"ancestor_record_offset":16508,"inlinee":7418,"source_line":692,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21560,"parent_record_offset":21524,"ancestor_record_offset":16508,"inlinee":7417,"source_line":615,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":21624,"parent_record_offset":21596,"ancestor_record_offset":16508,"inlinee":7419,"source_line":709,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":22248,"parent_record_offset":22216,"ancestor_record_offset":16508,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":22216,"parent_record_offset":22188,"ancestor_record_offset":16508,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":26592,"parent_record_offset":26568,"ancestor_record_offset":16508,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":22280,"parent_record_offset":22248,"ancestor_record_offset":16508,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":707,"record_offset":38624,"parent_record_offset":38584,"ancestor_record_offset":16508,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":707,"record_offset":38584,"parent_record_offset":38556,"ancestor_record_offset":16508,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":19736,"parent_record_offset":19696,"ancestor_record_offset":16508,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30224,"parent_record_offset":30200,"ancestor_record_offset":16508,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":16508,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":22552,"parent_record_offset":22524,"ancestor_record_offset":16508,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":21448,"parent_record_offset":21408,"ancestor_record_offset":16508,"inlinee":7413,"source_line":1471,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":19668,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":19696,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":21488,"parent_record_offset":21448,"ancestor_record_offset":16508,"inlinee":7414,"source_line":1407,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":22132,"parent_record_offset":22088,"ancestor_record_offset":16508,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":22160,"parent_record_offset":22132,"ancestor_record_offset":16508,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":23052,"parent_record_offset":23012,"ancestor_record_offset":16508,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":22008,"parent_record_offset":21968,"ancestor_record_offset":16508,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":16508,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":20200,"parent_record_offset":20164,"ancestor_record_offset":16508,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":22760,"parent_record_offset":22720,"ancestor_record_offset":16508,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":16508,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30152,"parent_record_offset":30120,"ancestor_record_offset":16508,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":21408,"parent_record_offset":21368,"ancestor_record_offset":16508,"inlinee":7412,"source_line":4002,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":16508,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":707,"record_offset":26324,"parent_record_offset":26292,"ancestor_record_offset":16508,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":707,"record_offset":26292,"parent_record_offset":26264,"ancestor_record_offset":16508,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":707,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":16508,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":707,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":16508,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":707,"record_offset":22852,"parent_record_offset":22760,"ancestor_record_offset":16508,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":707,"record_offset":22524,"parent_record_offset":22088,"ancestor_record_offset":16508,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":707,"record_offset":22048,"parent_record_offset":22008,"ancestor_record_offset":16508,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":707,"record_offset":22088,"parent_record_offset":22048,"ancestor_record_offset":16508,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":30348,"parent_record_offset":30120,"ancestor_record_offset":16508,"inlinee":39961,"source_line":2464,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30380,"parent_record_offset":30348,"ancestor_record_offset":16508,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30404,"parent_record_offset":30380,"ancestor_record_offset":16508,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":32588,"parent_record_offset":32552,"ancestor_record_offset":16508,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":16508,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":16508,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":707,"record_offset":33752,"parent_record_offset":33724,"ancestor_record_offset":16508,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":707,"record_offset":22304,"parent_record_offset":22280,"ancestor_record_offset":16508,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":707,"record_offset":22328,"parent_record_offset":22304,"ancestor_record_offset":16508,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18712,"parent_record_offset":18684,"ancestor_record_offset":16508,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":32652,"parent_record_offset":32552,"ancestor_record_offset":16508,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18764,"parent_record_offset":18740,"ancestor_record_offset":16508,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":16508,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18652,"parent_record_offset":18620,"ancestor_record_offset":16508,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":16508,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18856,"parent_record_offset":18684,"ancestor_record_offset":16508,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":18968,"parent_record_offset":18944,"ancestor_record_offset":16508,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":707,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":16508,"inlinee":14050,"source_line":1502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":16508,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":707,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":16508,"inlinee":10145,"source_line":118,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":707,"record_offset":23424,"parent_record_offset":23396,"ancestor_record_offset":16508,"inlinee":10146,"source_line":101,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":24224,"parent_record_offset":24064,"ancestor_record_offset":16508,"inlinee":18118,"source_line":39,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":24016,"parent_record_offset":23336,"ancestor_record_offset":16508,"inlinee":18108,"source_line":142,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":707,"record_offset":23368,"parent_record_offset":23336,"ancestor_record_offset":16508,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":707,"record_offset":22608,"parent_record_offset":22580,"ancestor_record_offset":16508,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":707,"record_offset":22680,"parent_record_offset":22636,"ancestor_record_offset":16508,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":707,"record_offset":27384,"parent_record_offset":27304,"ancestor_record_offset":16508,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":707,"record_offset":27408,"parent_record_offset":27384,"ancestor_record_offset":16508,"inlinee":5101,"source_line":155,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":707,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":16508,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":707,"record_offset":26372,"parent_record_offset":26348,"ancestor_record_offset":16508,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":707,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":16508,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23396,"parent_record_offset":23368,"ancestor_record_offset":16508,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23512,"parent_record_offset":23484,"ancestor_record_offset":16508,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":22580,"parent_record_offset":22552,"ancestor_record_offset":16508,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":22720,"parent_record_offset":22680,"ancestor_record_offset":16508,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23280,"parent_record_offset":23152,"ancestor_record_offset":16508,"inlinee":39955,"source_line":3143,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23308,"parent_record_offset":23280,"ancestor_record_offset":16508,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":24640,"parent_record_offset":24224,"ancestor_record_offset":16508,"inlinee":14049,"source_line":3265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23336,"parent_record_offset":23308,"ancestor_record_offset":16508,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":707,"record_offset":23456,"parent_record_offset":23424,"ancestor_record_offset":16508,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":30444,"parent_record_offset":29560,"ancestor_record_offset":16508,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":18788,"parent_record_offset":18764,"ancestor_record_offset":16508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":33776,"parent_record_offset":33752,"ancestor_record_offset":16508,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":29780,"parent_record_offset":29688,"ancestor_record_offset":16508,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":16508,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":16508,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":21692,"parent_record_offset":21668,"ancestor_record_offset":16508,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":707,"record_offset":17048,"parent_record_offset":17016,"ancestor_record_offset":16508,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":707,"record_offset":18488,"parent_record_offset":18448,"ancestor_record_offset":16508,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":707,"record_offset":23484,"parent_record_offset":23456,"ancestor_record_offset":16508,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":24608,"parent_record_offset":24252,"ancestor_record_offset":16508,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":24724,"parent_record_offset":23152,"ancestor_record_offset":16508,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":17296,"parent_record_offset":17268,"ancestor_record_offset":16508,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":18588,"parent_record_offset":18448,"ancestor_record_offset":16508,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":707,"record_offset":24188,"parent_record_offset":24164,"ancestor_record_offset":16508,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":18920,"parent_record_offset":18620,"ancestor_record_offset":16508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":32616,"parent_record_offset":32588,"ancestor_record_offset":16508,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":20252,"parent_record_offset":20224,"ancestor_record_offset":16508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":24164,"parent_record_offset":24140,"ancestor_record_offset":16508,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":16508,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":38656,"parent_record_offset":38624,"ancestor_record_offset":16508,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":17760,"parent_record_offset":17736,"ancestor_record_offset":16508,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":25816,"parent_record_offset":25728,"ancestor_record_offset":16508,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":17736,"parent_record_offset":17712,"ancestor_record_offset":16508,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":24140,"parent_record_offset":24064,"ancestor_record_offset":16508,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":16508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":25132,"parent_record_offset":25100,"ancestor_record_offset":16508,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":18812,"parent_record_offset":18788,"ancestor_record_offset":16508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":21808,"parent_record_offset":21624,"ancestor_record_offset":16508,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":707,"record_offset":23196,"parent_record_offset":23152,"ancestor_record_offset":16508,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":23080,"parent_record_offset":23052,"ancestor_record_offset":16508,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":707,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":16508,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":707,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":16508,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":17688,"parent_record_offset":17664,"ancestor_record_offset":16508,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":16508,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":17712,"parent_record_offset":17688,"ancestor_record_offset":16508,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":16508,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":18016,"parent_record_offset":17988,"ancestor_record_offset":16508,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":22636,"parent_record_offset":22608,"ancestor_record_offset":16508,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":16508,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":17664,"parent_record_offset":17548,"ancestor_record_offset":16508,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":23772,"parent_record_offset":23708,"ancestor_record_offset":16508,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":24984,"parent_record_offset":24920,"ancestor_record_offset":16508,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":707,"record_offset":17016,"parent_record_offset":16988,"ancestor_record_offset":16508,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":707,"record_offset":17376,"parent_record_offset":17348,"ancestor_record_offset":16508,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":707,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":16508,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":707,"record_offset":17484,"parent_record_offset":17448,"ancestor_record_offset":16508,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":707,"record_offset":17424,"parent_record_offset":17400,"ancestor_record_offset":16508,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":707,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":16508,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":707,"record_offset":17400,"parent_record_offset":17376,"ancestor_record_offset":16508,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":707,"record_offset":17988,"parent_record_offset":17448,"ancestor_record_offset":16508,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":16988,"parent_record_offset":16956,"ancestor_record_offset":16508,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":16508,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":17148,"parent_record_offset":17116,"ancestor_record_offset":16508,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":17244,"parent_record_offset":17148,"ancestor_record_offset":16508,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":16508,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":16932,"parent_record_offset":16900,"ancestor_record_offset":16508,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":707,"record_offset":17268,"parent_record_offset":17244,"ancestor_record_offset":16508,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":707,"record_offset":17548,"parent_record_offset":17516,"ancestor_record_offset":16508,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":707,"record_offset":17832,"parent_record_offset":17548,"ancestor_record_offset":16508,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":707,"record_offset":17804,"parent_record_offset":17548,"ancestor_record_offset":16508,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":707,"record_offset":17608,"parent_record_offset":17548,"ancestor_record_offset":16508,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30592,"parent_record_offset":30528,"ancestor_record_offset":16508,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":19428,"parent_record_offset":19392,"ancestor_record_offset":16508,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29808,"parent_record_offset":29688,"ancestor_record_offset":16508,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30876,"parent_record_offset":30748,"ancestor_record_offset":16508,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29724,"parent_record_offset":29688,"ancestor_record_offset":16508,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30616,"parent_record_offset":30592,"ancestor_record_offset":16508,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":19452,"parent_record_offset":19428,"ancestor_record_offset":16508,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":16508,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":16508,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":38716,"parent_record_offset":38584,"ancestor_record_offset":16508,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":29964,"parent_record_offset":29560,"ancestor_record_offset":16508,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":30500,"parent_record_offset":30472,"ancestor_record_offset":16508,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":29628,"parent_record_offset":29560,"ancestor_record_offset":16508,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":30720,"parent_record_offset":30692,"ancestor_record_offset":16508,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":19392,"parent_record_offset":19364,"ancestor_record_offset":16508,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":30528,"parent_record_offset":30500,"ancestor_record_offset":16508,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29868,"parent_record_offset":29652,"ancestor_record_offset":16508,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":30788,"parent_record_offset":30748,"ancestor_record_offset":16508,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29688,"parent_record_offset":29652,"ancestor_record_offset":16508,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":16508,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":30748,"parent_record_offset":30720,"ancestor_record_offset":16508,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":19484,"parent_record_offset":19392,"ancestor_record_offset":16508,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":30564,"parent_record_offset":30528,"ancestor_record_offset":16508,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":18364,"parent_record_offset":18324,"ancestor_record_offset":16508,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":31184,"parent_record_offset":31148,"ancestor_record_offset":16508,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":31752,"parent_record_offset":31480,"ancestor_record_offset":16508,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":707,"record_offset":18516,"parent_record_offset":18488,"ancestor_record_offset":16508,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":18620,"parent_record_offset":18448,"ancestor_record_offset":16508,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":707,"record_offset":18892,"parent_record_offset":18620,"ancestor_record_offset":16508,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::update_pending_submission_timestamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":24752,"parent_record_offset":23152,"ancestor_record_offset":16508,"inlinee":39327,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":38152,"parent_record_offset":38120,"ancestor_record_offset":16508,"inlinee":39301,"source_line":125,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":707,"record_offset":31028,"parent_record_offset":31004,"ancestor_record_offset":16508,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmitInfo::command_buffers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":20164,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":39950,"source_line":30,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmitInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":20108,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":39505,"source_line":21,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmitInfo::signal_semaphore
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":20700,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":39952,"source_line":57,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmitInfo::wait_semaphore
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":20292,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":39951,"source_line":35,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":23568,"parent_record_offset":23540,"ancestor_record_offset":16508,"inlinee":39957,"source_line":110,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::queue_submit::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":30120,"parent_record_offset":29560,"ancestor_record_offset":16508,"inlinee":39960,"source_line":342,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::queue_submit::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":30980,"parent_record_offset":30952,"ancestor_record_offset":16508,"inlinee":39962,"source_line":346,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":16508,"inlinee":39507,"source_line":10,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":19880,"parent_record_offset":19520,"ancestor_record_offset":16508,"inlinee":39627,"source_line":76,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":707,"record_offset":18544,"parent_record_offset":18516,"ancestor_record_offset":16508,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":707,"record_offset":18448,"parent_record_offset":18404,"ancestor_record_offset":16508,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":707,"record_offset":18404,"parent_record_offset":18364,"ancestor_record_offset":16508,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F1
// 首条位置: {"module":1936,"inlinee":86769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1936,"record_offset":29264,"parent_record_offset":29244,"ancestor_record_offset":29244,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1936,"record_offset":29312,"parent_record_offset":29288,"ancestor_record_offset":29244,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1936,"record_offset":29288,"parent_record_offset":29264,"ancestor_record_offset":29244,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F3
// 首条位置: {"module":1936,"inlinee":86771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1936,"record_offset":43424,"parent_record_offset":43404,"ancestor_record_offset":43404,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1936,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":43404,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1936,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":43404,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F5
// 首条位置: {"module":1936,"inlinee":86773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1936,"record_offset":57024,"parent_record_offset":57004,"ancestor_record_offset":57004,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1936,"record_offset":57072,"parent_record_offset":57048,"ancestor_record_offset":57004,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1936,"record_offset":57048,"parent_record_offset":57024,"ancestor_record_offset":57004,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F4
// 首条位置: {"module":1936,"inlinee":86772}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1936,"record_offset":48264,"parent_record_offset":48244,"ancestor_record_offset":48244,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1936,"record_offset":48312,"parent_record_offset":48288,"ancestor_record_offset":48244,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1936,"record_offset":48288,"parent_record_offset":48264,"ancestor_record_offset":48244,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152DB
// 首条位置: {"module":1935,"inlinee":86747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::SubmitInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":17484,"parent_record_offset":16056,"ancestor_record_offset":16056,"inlinee":86751,"source_line":8028,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16144,"parent_record_offset":16056,"ancestor_record_offset":16056,"inlinee":15485,"source_line":1028,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::carve_off_front
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":1935,"record_offset":16700,"parent_record_offset":16056,"ancestor_record_offset":16056,"inlinee":86749,"source_line":70,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1935,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":16056,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16208,"parent_record_offset":16172,"ancestor_record_offset":16056,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16172,"parent_record_offset":16144,"ancestor_record_offset":16056,"inlinee":15486,"source_line":1071,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16236,"parent_record_offset":16208,"ancestor_record_offset":16056,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16276,"parent_record_offset":16236,"ancestor_record_offset":16056,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16600,"parent_record_offset":16236,"ancestor_record_offset":16056,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_with::inner_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16636,"parent_record_offset":16172,"ancestor_record_offset":16056,"inlinee":15492,"source_line":1076,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1935,"record_offset":16372,"parent_record_offset":16276,"ancestor_record_offset":16056,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1935,"record_offset":16316,"parent_record_offset":16276,"ancestor_record_offset":16056,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1935,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":16056,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1935,"record_offset":16456,"parent_record_offset":16276,"ancestor_record_offset":16056,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1935,"record_offset":16860,"parent_record_offset":16736,"ancestor_record_offset":16056,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1935,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":16056,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":17556,"parent_record_offset":17532,"ancestor_record_offset":16056,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":16056,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":16056,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":16056,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1935,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":16056,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1935,"record_offset":16572,"parent_record_offset":16236,"ancestor_record_offset":16056,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1935,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":16056,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1935,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":16056,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1935,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":16056,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1935,"record_offset":16528,"parent_record_offset":16504,"ancestor_record_offset":16056,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16736,"parent_record_offset":16700,"ancestor_record_offset":16056,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16772,"parent_record_offset":16736,"ancestor_record_offset":16056,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1935,"record_offset":16800,"parent_record_offset":16772,"ancestor_record_offset":16056,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C18
// 首条位置: {"module":707,"inlinee":39960}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30152,"parent_record_offset":30120,"ancestor_record_offset":30120,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":30348,"parent_record_offset":30120,"ancestor_record_offset":30120,"inlinee":39961,"source_line":2464,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30248,"parent_record_offset":30224,"ancestor_record_offset":30120,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30296,"parent_record_offset":30272,"ancestor_record_offset":30120,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30272,"parent_record_offset":30248,"ancestor_record_offset":30120,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30224,"parent_record_offset":30200,"ancestor_record_offset":30120,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":30120,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":30120,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30380,"parent_record_offset":30348,"ancestor_record_offset":30120,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30404,"parent_record_offset":30380,"ancestor_record_offset":30120,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::queue_submit::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C1A
// 首条位置: {"module":707,"inlinee":39962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":707,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":30980,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":707,"record_offset":31028,"parent_record_offset":31004,"ancestor_record_offset":30980,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_submit_done
// 记录形式: compiled_procedure; instances=1
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":121384,"rva":12568448,"va":5381277568}
// 编译实例 RVAs: 0xbfc780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11121206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8ca4d56ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381279329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7649643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e895468bff","size":5,"soft_fail":false,"target_va":5373629488,"va":5381279126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5763046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e81a10a8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381279073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4383795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8cd1bbdff","size":5,"soft_fail":false,"target_va":5376895600,"va":5381279390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-53035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8d530ffff","size":5,"soft_fail":false,"target_va":5381225824,"va":5381278854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t553909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8b5730800","size":5,"soft_fail":false,"target_va":5381832720,"va":5381278806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30262855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e847c6cd01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381278724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30263272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8e8c7cd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381279019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30263870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e83ecacd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381279005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-61324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e87410ffff","size":5,"soft_fail":false,"target_va":5381216320,"va":5381277639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30265646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e82ed1cd01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381277709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13388144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e87049cc00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381278219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30213625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8f905cd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381278738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30216042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e86a0fcd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381278753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t545343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e83f520800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381278252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t545607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e847530800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381278404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t107367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e867a30100","size":5,"soft_fail":false,"target_va":5381385536,"va":5381278164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t112556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8acb70100","size":5,"soft_fail":false,"target_va":5381390640,"va":5381278079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t107849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e849a50100","size":5,"soft_fail":false,"target_va":5381386480,"va":5381278626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t107089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e851a20100","size":5,"soft_fail":false,"target_va":5381384704,"va":5381277610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-48715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8b541ffff","size":5,"soft_fail":false,"target_va":5381229760,"va":5381278470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13385610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e88a3fcc00","size":5,"soft_fail":false,"target_va":5394664048,"va":5381278433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30459137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e801c5d001","size":5,"soft_fail":false,"target_va":5411737808,"va":5381278666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13393881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8d95fcc00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381279266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30258387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381277568,"raw_hex":"e8d3b4cd01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381278712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1272(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381277568,"raw_hex":"ff96f8040000","size":6,"soft_fail":false,"va":5381278387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::wait_semaphores
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":124404,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39758,"source_line":691,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreWaitInfo::semaphores
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":124348,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39756,"source_line":21994,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SemaphoreWaitInfo::values
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":124376,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39757,"source_line":22000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1827::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":124180,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39753,"source_line":21972,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":124864,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":124488,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":126720,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":124024,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":123008,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":121576,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":122032,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::wait_submit_done
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":123992,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39355,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":124208,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39754,"source_line":144,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":125208,"parent_record_offset":121384,"ancestor_record_offset":121384,"inlinee":39307,"source_line":135,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":125288,"parent_record_offset":125264,"ancestor_record_offset":121384,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":124248,"parent_record_offset":124208,"ancestor_record_offset":121384,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":125240,"parent_record_offset":125208,"ancestor_record_offset":121384,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":125264,"parent_record_offset":125240,"ancestor_record_offset":121384,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":124428,"parent_record_offset":124404,"ancestor_record_offset":121384,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":124452,"parent_record_offset":124428,"ancestor_record_offset":121384,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123344,"parent_record_offset":123316,"ancestor_record_offset":121384,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123396,"parent_record_offset":123372,"ancestor_record_offset":121384,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123372,"parent_record_offset":123344,"ancestor_record_offset":121384,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123292,"parent_record_offset":123264,"ancestor_record_offset":121384,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123316,"parent_record_offset":123292,"ancestor_record_offset":121384,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123488,"parent_record_offset":123316,"ancestor_record_offset":121384,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":123600,"parent_record_offset":123576,"ancestor_record_offset":121384,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":124788,"parent_record_offset":124664,"ancestor_record_offset":121384,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":703,"record_offset":124560,"parent_record_offset":124528,"ancestor_record_offset":121384,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":124820,"parent_record_offset":124664,"ancestor_record_offset":121384,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":123420,"parent_record_offset":123396,"ancestor_record_offset":121384,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":703,"record_offset":121732,"parent_record_offset":121700,"ancestor_record_offset":121384,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":123160,"parent_record_offset":123116,"ancestor_record_offset":121384,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":121980,"parent_record_offset":121952,"ancestor_record_offset":121384,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":123948,"parent_record_offset":123116,"ancestor_record_offset":121384,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":123552,"parent_record_offset":123264,"ancestor_record_offset":121384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":125448,"parent_record_offset":125424,"ancestor_record_offset":121384,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":122444,"parent_record_offset":122420,"ancestor_record_offset":121384,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":122420,"parent_record_offset":122396,"ancestor_record_offset":121384,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":122628,"parent_record_offset":122600,"ancestor_record_offset":121384,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":123444,"parent_record_offset":123420,"ancestor_record_offset":121384,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":124276,"parent_record_offset":124248,"ancestor_record_offset":121384,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":124304,"parent_record_offset":124276,"ancestor_record_offset":121384,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":122372,"parent_record_offset":122348,"ancestor_record_offset":121384,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":122396,"parent_record_offset":122372,"ancestor_record_offset":121384,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":125424,"parent_record_offset":125400,"ancestor_record_offset":121384,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":122700,"parent_record_offset":122672,"ancestor_record_offset":121384,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":122348,"parent_record_offset":122232,"ancestor_record_offset":121384,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":125324,"parent_record_offset":125208,"ancestor_record_offset":121384,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":125400,"parent_record_offset":125324,"ancestor_record_offset":121384,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":121700,"parent_record_offset":121672,"ancestor_record_offset":121384,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":122060,"parent_record_offset":122032,"ancestor_record_offset":121384,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":122200,"parent_record_offset":122168,"ancestor_record_offset":121384,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":122168,"parent_record_offset":122132,"ancestor_record_offset":121384,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":122108,"parent_record_offset":122084,"ancestor_record_offset":121384,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":122132,"parent_record_offset":122108,"ancestor_record_offset":121384,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":122084,"parent_record_offset":122060,"ancestor_record_offset":121384,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":122672,"parent_record_offset":122132,"ancestor_record_offset":121384,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121672,"parent_record_offset":121640,"ancestor_record_offset":121384,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121640,"parent_record_offset":121612,"ancestor_record_offset":121384,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121832,"parent_record_offset":121800,"ancestor_record_offset":121384,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121928,"parent_record_offset":121832,"ancestor_record_offset":121384,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121800,"parent_record_offset":121776,"ancestor_record_offset":121384,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121612,"parent_record_offset":121576,"ancestor_record_offset":121384,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":121952,"parent_record_offset":121928,"ancestor_record_offset":121384,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":122232,"parent_record_offset":122200,"ancestor_record_offset":121384,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":122516,"parent_record_offset":122232,"ancestor_record_offset":121384,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":122488,"parent_record_offset":122232,"ancestor_record_offset":121384,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":122292,"parent_record_offset":122232,"ancestor_record_offset":121384,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124984,"parent_record_offset":124920,"ancestor_record_offset":121384,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124088,"parent_record_offset":124052,"ancestor_record_offset":121384,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124728,"parent_record_offset":124704,"ancestor_record_offset":121384,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":125008,"parent_record_offset":124984,"ancestor_record_offset":121384,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124112,"parent_record_offset":124088,"ancestor_record_offset":121384,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124752,"parent_record_offset":124728,"ancestor_record_offset":121384,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":124892,"parent_record_offset":124864,"ancestor_record_offset":121384,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":124704,"parent_record_offset":124664,"ancestor_record_offset":121384,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":124052,"parent_record_offset":124024,"ancestor_record_offset":121384,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":124920,"parent_record_offset":124892,"ancestor_record_offset":121384,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":124144,"parent_record_offset":124052,"ancestor_record_offset":121384,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":124956,"parent_record_offset":124920,"ancestor_record_offset":121384,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":123044,"parent_record_offset":123008,"ancestor_record_offset":121384,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":125744,"parent_record_offset":125712,"ancestor_record_offset":121384,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":126244,"parent_record_offset":125984,"ancestor_record_offset":121384,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":703,"record_offset":123188,"parent_record_offset":123160,"ancestor_record_offset":121384,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":123264,"parent_record_offset":123116,"ancestor_record_offset":121384,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":123524,"parent_record_offset":123264,"ancestor_record_offset":121384,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::log_error_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":703,"record_offset":124620,"parent_record_offset":124528,"ancestor_record_offset":121384,"inlinee":39761,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::log_vulkan_error_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":703,"record_offset":124664,"parent_record_offset":124620,"ancestor_record_offset":121384,"inlinee":39616,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":703,"record_offset":124584,"parent_record_offset":124560,"ancestor_record_offset":121384,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":124528,"parent_record_offset":124488,"ancestor_record_offset":121384,"inlinee":39760,"source_line":381,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":703,"record_offset":123216,"parent_record_offset":123188,"ancestor_record_offset":121384,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":123116,"parent_record_offset":123080,"ancestor_record_offset":121384,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":123080,"parent_record_offset":123044,"ancestor_record_offset":121384,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B50
// 首条位置: {"module":703,"inlinee":39760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":703,"record_offset":124560,"parent_record_offset":124528,"ancestor_record_offset":124528,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::log_error_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":703,"record_offset":124620,"parent_record_offset":124528,"ancestor_record_offset":124528,"inlinee":39761,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":124788,"parent_record_offset":124664,"ancestor_record_offset":124528,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":124820,"parent_record_offset":124664,"ancestor_record_offset":124528,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124728,"parent_record_offset":124704,"ancestor_record_offset":124528,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":124752,"parent_record_offset":124728,"ancestor_record_offset":124528,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":124704,"parent_record_offset":124664,"ancestor_record_offset":124528,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::log_vulkan_error_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":703,"record_offset":124664,"parent_record_offset":124620,"ancestor_record_offset":124528,"inlinee":39616,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":703,"record_offset":124584,"parent_record_offset":124560,"ancestor_record_offset":124528,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done
// 记录形式: compiled_procedure; instances=1
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":126868,"rva":12570496,"va":5381279616}
// 编译实例 RVAs: 0xbfcf80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Sub$GT$::sub
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1468354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8c2671600","size":5,"soft_fail":false,"target_va":5382748928,"va":5381280569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11123349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e86b4556ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381281472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30564512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8a060d201","size":5,"soft_fail":false,"target_va":5411845280,"va":5381280763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: submit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30564487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e88760d201","size":5,"soft_fail":false,"target_va":5411845296,"va":5381280804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30260478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8febccd01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381281101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30260716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8ecbdcd01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381281087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30261088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e860bfcd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381281203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30261690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8bac1cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381281185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-63382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e86a08ffff","size":5,"soft_fail":false,"target_va":5381216320,"va":5381279697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30263588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e824c9cd01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381279767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30457689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e859bfd001","size":5,"soft_fail":false,"target_va":5411738448,"va":5381280754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30211504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8b0fdcc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381280859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30213955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e84307cd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381280840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t543130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e89a490800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381280465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t543195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8db490800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381280816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t105287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8479b0100","size":5,"soft_fail":false,"target_va":5381385536,"va":5381280244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t110476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e88caf0100","size":5,"soft_fail":false,"target_va":5381390640,"va":5381280159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t106130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8929e0100","size":5,"soft_fail":false,"target_va":5381386480,"va":5381280345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t105036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e84c9a0100","size":5,"soft_fail":false,"target_va":5381384704,"va":5381279663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30456812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8ecbbd001","size":5,"soft_fail":false,"target_va":5411737808,"va":5381280991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13391869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8fd57cc00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381281278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30256036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e8a4abcd01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381281063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1410421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e875851500","size":5,"soft_fail":false,"target_va":5382691200,"va":5381280774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1467256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381279616,"raw_hex":"e878631600","size":5,"soft_fail":false,"target_va":5382747616,"va":5381280355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1264(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381279616,"raw_hex":"ff93f0040000","size":6,"soft_fail":false,"va":5381280525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::get_semaphore_counter_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":130732,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":39621,"source_line":679,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":130848,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":127528,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":129584,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":133372,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::time::impl$3::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":703,"record_offset":130940,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":39763,"source_line":1125,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":130444,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":128600,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":127072,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":127624,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":130600,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":39754,"source_line":144,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] spin_sleep::native_sleep
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"spin_sleep","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/spin_sleep-1.1.1/src/lib.rs
//     evidence={"module":703,"record_offset":131244,"parent_record_offset":126868,"ancestor_record_offset":126868,"inlinee":11242,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":130632,"parent_record_offset":130600,"ancestor_record_offset":126868,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":130760,"parent_record_offset":130732,"ancestor_record_offset":126868,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":130784,"parent_record_offset":130760,"ancestor_record_offset":126868,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":130808,"parent_record_offset":130784,"ancestor_record_offset":126868,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":128936,"parent_record_offset":128908,"ancestor_record_offset":126868,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":128988,"parent_record_offset":128964,"ancestor_record_offset":126868,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":128964,"parent_record_offset":128936,"ancestor_record_offset":126868,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":128884,"parent_record_offset":128856,"ancestor_record_offset":126868,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":128908,"parent_record_offset":128884,"ancestor_record_offset":126868,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":129080,"parent_record_offset":128908,"ancestor_record_offset":126868,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":129192,"parent_record_offset":129168,"ancestor_record_offset":126868,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":127584,"parent_record_offset":127556,"ancestor_record_offset":126868,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":131144,"parent_record_offset":131120,"ancestor_record_offset":126868,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":129012,"parent_record_offset":128988,"ancestor_record_offset":126868,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":703,"record_offset":127228,"parent_record_offset":127196,"ancestor_record_offset":126868,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":131120,"parent_record_offset":131076,"ancestor_record_offset":126868,"inlinee":7229,"source_line":486,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":131016,"parent_record_offset":130976,"ancestor_record_offset":126868,"inlinee":18849,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":128752,"parent_record_offset":128708,"ancestor_record_offset":126868,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":131176,"parent_record_offset":131076,"ancestor_record_offset":126868,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":127476,"parent_record_offset":127448,"ancestor_record_offset":126868,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":129540,"parent_record_offset":128708,"ancestor_record_offset":126868,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":129144,"parent_record_offset":128856,"ancestor_record_offset":126868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":128036,"parent_record_offset":128012,"ancestor_record_offset":126868,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":128012,"parent_record_offset":127988,"ancestor_record_offset":126868,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":128220,"parent_record_offset":128192,"ancestor_record_offset":126868,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":129036,"parent_record_offset":129012,"ancestor_record_offset":126868,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":130660,"parent_record_offset":130632,"ancestor_record_offset":126868,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":130688,"parent_record_offset":130660,"ancestor_record_offset":126868,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":127964,"parent_record_offset":127940,"ancestor_record_offset":126868,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":127988,"parent_record_offset":127964,"ancestor_record_offset":126868,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":128292,"parent_record_offset":128264,"ancestor_record_offset":126868,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":127940,"parent_record_offset":127824,"ancestor_record_offset":126868,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":127196,"parent_record_offset":127168,"ancestor_record_offset":126868,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":127652,"parent_record_offset":127624,"ancestor_record_offset":126868,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":127792,"parent_record_offset":127760,"ancestor_record_offset":126868,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":127760,"parent_record_offset":127724,"ancestor_record_offset":126868,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":127700,"parent_record_offset":127676,"ancestor_record_offset":126868,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":127724,"parent_record_offset":127700,"ancestor_record_offset":126868,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":127676,"parent_record_offset":127652,"ancestor_record_offset":126868,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":128264,"parent_record_offset":127724,"ancestor_record_offset":126868,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127168,"parent_record_offset":127136,"ancestor_record_offset":126868,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127136,"parent_record_offset":127108,"ancestor_record_offset":126868,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127328,"parent_record_offset":127296,"ancestor_record_offset":126868,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127424,"parent_record_offset":127328,"ancestor_record_offset":126868,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127296,"parent_record_offset":127272,"ancestor_record_offset":126868,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127108,"parent_record_offset":127072,"ancestor_record_offset":126868,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":127448,"parent_record_offset":127424,"ancestor_record_offset":126868,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":127824,"parent_record_offset":127792,"ancestor_record_offset":126868,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":128108,"parent_record_offset":127824,"ancestor_record_offset":126868,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":128080,"parent_record_offset":127824,"ancestor_record_offset":126868,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":127884,"parent_record_offset":127824,"ancestor_record_offset":126868,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":131452,"parent_record_offset":131424,"ancestor_record_offset":126868,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":131880,"parent_record_offset":131820,"ancestor_record_offset":126868,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":130508,"parent_record_offset":130472,"ancestor_record_offset":126868,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":131904,"parent_record_offset":131880,"ancestor_record_offset":126868,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":130532,"parent_record_offset":130508,"ancestor_record_offset":126868,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":131480,"parent_record_offset":131452,"ancestor_record_offset":126868,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":703,"record_offset":130976,"parent_record_offset":130940,"ancestor_record_offset":126868,"inlinee":18848,"source_line":693,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":703,"record_offset":131076,"parent_record_offset":130976,"ancestor_record_offset":126868,"inlinee":19137,"source_line":194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":703,"record_offset":127556,"parent_record_offset":127528,"ancestor_record_offset":126868,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":131796,"parent_record_offset":131772,"ancestor_record_offset":126868,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":703,"record_offset":131424,"parent_record_offset":131396,"ancestor_record_offset":126868,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":703,"record_offset":131308,"parent_record_offset":131280,"ancestor_record_offset":126868,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":703,"record_offset":131396,"parent_record_offset":131364,"ancestor_record_offset":126868,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":703,"record_offset":131336,"parent_record_offset":131308,"ancestor_record_offset":126868,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":703,"record_offset":131364,"parent_record_offset":131336,"ancestor_record_offset":126868,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":703,"record_offset":131280,"parent_record_offset":131244,"ancestor_record_offset":126868,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":130472,"parent_record_offset":130444,"ancestor_record_offset":126868,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":131820,"parent_record_offset":131796,"ancestor_record_offset":126868,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":130564,"parent_record_offset":130472,"ancestor_record_offset":126868,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":131852,"parent_record_offset":131820,"ancestor_record_offset":126868,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":128636,"parent_record_offset":128600,"ancestor_record_offset":126868,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":129648,"parent_record_offset":129616,"ancestor_record_offset":126868,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":130164,"parent_record_offset":129904,"ancestor_record_offset":126868,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":703,"record_offset":128780,"parent_record_offset":128752,"ancestor_record_offset":126868,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":128856,"parent_record_offset":128708,"ancestor_record_offset":126868,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":129116,"parent_record_offset":128856,"ancestor_record_offset":126868,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":703,"record_offset":128808,"parent_record_offset":128780,"ancestor_record_offset":126868,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":128708,"parent_record_offset":128672,"ancestor_record_offset":126868,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":128672,"parent_record_offset":128636,"ancestor_record_offset":126868,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: submit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::DeviceSubmissionTimestamp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<u64>", "rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$18::extend<rhapsody::raw::submit::DeviceSubmissionTimestamp,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)> >", "constructor": "alloc::collections::vec_deque::impl$18::extend", "arguments": ["rhapsody::raw::submit::DeviceSubmissionTimestamp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<u64>,rhapsody::raw::submit::DeviceSubmissionTimestamp (*)(u64)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::QueueIndex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::submit::QueueIndex>", "constructor": "enum2$", "arguments": ["rhapsody::raw::submit::QueueIndex"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panicking::assert_failed<enum2$<rhapsody::raw::submit::QueueIndex>,enum2$<rhapsody::raw::submit::QueueIndex> >", "constructor": "core::panicking::assert_failed", "arguments": ["enum2$<rhapsody::raw::submit::QueueIndex>", "enum2$<rhapsody::raw::submit::QueueIndex>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::submit::QueueIndex> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::submit::QueueIndex>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::SubmissionEngine
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::submit::SubmissionEngine"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::submit::SubmissionEngine"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::submit::SubmissionEngine>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::submit::SubmitInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::option::Option", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::raw::submit::SubmitInfo> >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::raw::submit::SubmitInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<rhapsody::raw::submit::SubmitInfo> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<rhapsody::raw::submit::SubmitInfo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["rhapsody::raw::submit::SubmitInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::submit::SubmitInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::device::GraphicsDevice::queue_submit", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::slice::iter::IterMut", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>", "alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["ash::vk::definitions::SubmitInfo", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["ash::vk::definitions::SubmitInfo", "alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["ash::vk::definitions::SubmitInfo", "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::submit::SubmitInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>", "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::CommandBuffer", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::Semaphore", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::Semaphore", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::bitflags::PipelineStageFlags", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::SubmitInfo,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::SubmitInfo", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: submit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: submit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::queue_family_index
// 信息 ID: c16dd4215479f4e446cdf8df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf84f5 -> 0xbfc750 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c327e -> 0xbfc750 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > > @ 0x16a328e -> 0xbfc750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>
// 信息 ID: 23f7298a18b5e9b08e783b71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["rhapsody::raw::submit::SubmitInfo", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::queue_submit -> ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs @ 0xbebfd6 -> 0xc04e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done
// 信息 ID: cf42a098f71aa9d726e9c18d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim @ 0xbea95f -> 0xbfcf80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_submit_done
// 信息 ID: 60b8989cb44a0cc640ae8a44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeaf6b -> 0xbfc780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::queue
// 信息 ID: 11d9c74444ab4a32256da530
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=703, inline_record=124208; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done; module=703, inline_record=130600; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1987, inline_record=26000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::queue_mut
// 信息 ID: d72d2d1430f690697da12e5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=23012; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=24788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
// 信息 ID: 7f57e8b2a8b34d1bb235b9d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::impl$5::drop; module=690, inline_record=102564; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources; module=703, inline_record=70844; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=703, inline_record=125208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::update_pending_submission_timestamp
// 信息 ID: f0df904f0adf194d21e28783
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=23152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
// 信息 ID: 97bce4830ffa300d9b19e319
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xbf8ebc -> 0x18c3300 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> @ 0xc0561e -> 0x18c3300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::command_buffer
// 信息 ID: bafeb0e1b84fc2746322489d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::begin_frame; module=699, inline_record=19536; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=703, inline_record=65108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::command_buffers
// 信息 ID: 5fd5fa8535cbc17f9d05c2b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=20164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::new
// 信息 ID: a436f3738483455d84a446ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::begin_frame; module=699, inline_record=19480; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=703, inline_record=65052; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=20108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::signal_semaphore
// 信息 ID: 092d605b737434a5becee1c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=20700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::SubmitInfo::wait_semaphore
// 信息 ID: 14ed8b98dd8e513553577a14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=20292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::carve_off_front
// 信息 ID: 26992994edd05b374d5573a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; module=1935, inline_record=16700; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; module=1935, inline_record=16896; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; module=1935, inline_record=17092; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; module=1935, inline_record=17288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$17::partial_cmp
// 信息 ID: 70042199481ac9523c70b89a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::update_pending_submissions; module=703, inline_record=28080; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=703, inline_record=31168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$1::update_pending_submission_timestamp::closure$0
// 信息 ID: 7d0eb878b7e34e4e1a8d8ef3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::submit::impl$1::update_pending_submission_timestamp", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=23568; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=23908; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=24252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$1
// 信息 ID: 0b91842da5ce3fef39aa6151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=29244; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=31576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$2
// 信息 ID: 1496d28605ac4e9d16496c27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=43404; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$2<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=45564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$3
// 信息 ID: f85fa40170fa08da998ed68a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=57004; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::bitflags::PipelineStageFlags,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::bitflags::PipelineStageFlags> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$3<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=59164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$4
// 信息 ID: 7678b9fe0866fec11905d211
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=48244; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<ash::vk::definitions::Semaphore> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$4<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; module=1936, inline_record=50404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$5
// 信息 ID: dca9eab9a6db67af876310e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::submit::impl$2::queue_submit::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::SubmitInfo,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::SubmitInfo,core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::SubmitInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<rhapsody::raw::submit::SubmitInfo>,alloc::vec::into_iter::IntoIter<usize,alloc::alloc::Global> >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$5<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> > > > >; module=1935, inline_record=16056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$6
// 信息 ID: ab606ff96089513132ab3a73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=30120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::queue_submit::closure$7
// 信息 ID: 36f3b66eb332ad73a04a2975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "rhapsody::raw::submit::impl$2::queue_submit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=30980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$2::wait_submit_done::closure$0
// 信息 ID: ca3ad56d10a1e2fd47c6d191
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::submit::impl$2::wait_submit_done", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=703, inline_record=124528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$3::default
// 信息 ID: 392591aa9df99bdbaef33fe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::begin_frame; module=699, inline_record=19504; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=703, inline_record=65076; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=20132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$7::eq
// 信息 ID: eaa0078a9d37298b33076df2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=703, inline_record=40664; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=19880; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=25876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::submit::impl$9::fmt
// 信息 ID: 67b5f5e540bb2a06e71a2cc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::submit::QueueIndex> >; module=1953, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
