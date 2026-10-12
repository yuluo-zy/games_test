// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/disposal.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e0ad82ba57df266472d6b7a5445230c9a7343bce142c7113d63d061e7e0e267a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: disposal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 684, 703, 1920, 1921, 1944, 1945, 1962, 1974, 1978, 1987
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":27508,"rva":25983712,"va":5394692832}
// 编译实例 RVAs: 0x18c7ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140511380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20671042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8be95c4fe","size":5,"soft_fail":false,"target_va":5374022528,"va":5394693565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfcf0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13533489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8cf7e31ff","size":5,"soft_fail":false,"target_va":5381160176,"va":5394693660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf92f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13429544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8d81433ff","size":5,"soft_fail":false,"target_va":5381264112,"va":5394693651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::resize_with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e89dc1ffff","size":5,"soft_fail":false,"target_va":5394677568,"va":5394693534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16847241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e889110101","size":5,"soft_fail":false,"target_va":5411541584,"va":5394694338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16848181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e835150101","size":5,"soft_fail":false,"target_va":5411542484,"va":5394694298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16847735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e877130101","size":5,"soft_fail":false,"target_va":5411542064,"va":5394694324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e852d6ffff","size":5,"soft_fail":false,"target_va":5394682240,"va":5394692905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16850373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8c51d0101","size":5,"soft_fail":false,"target_va":5411543360,"va":5394692982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e88a95ffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394693617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13307885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e813f034ff","size":5,"soft_fail":false,"target_va":5381385536,"va":5394693416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13302696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8580435ff","size":5,"soft_fail":false,"target_va":5381390640,"va":5394693331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13307462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8baf134ff","size":5,"soft_fail":false,"target_va":5381386480,"va":5394693937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13308180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8ecee34ff","size":5,"soft_fail":false,"target_va":5381384704,"va":5394692879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e8f4c2ffff","size":5,"soft_fail":false,"target_va":5394678016,"va":5394693639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17043613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e89d100401","size":5,"soft_fail":false,"target_va":5411737808,"va":5394694190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e84cacffff","size":5,"soft_fail":false,"target_va":5394673152,"va":5394694575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16842846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394692832,"raw_hex":"e85e000101","size":5,"soft_fail":false,"target_va":5411537104,"va":5394694253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":32616,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":31152,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30444,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30224,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":30192,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":7153,"source_line":1836,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1978,"record_offset":31476,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":31560,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":29208,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":27776,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":28232,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1978,"record_offset":30368,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":87095,"source_line":185,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":33084,"parent_record_offset":32960,"ancestor_record_offset":27508,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30276,"parent_record_offset":30252,"ancestor_record_offset":27508,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":32644,"parent_record_offset":32616,"ancestor_record_offset":27508,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30480,"parent_record_offset":30444,"ancestor_record_offset":27508,"inlinee":4906,"source_line":1342,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30252,"parent_record_offset":30224,"ancestor_record_offset":27508,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":31024,"parent_record_offset":30856,"ancestor_record_offset":27508,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30508,"parent_record_offset":30444,"ancestor_record_offset":27508,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1978,"record_offset":30536,"parent_record_offset":30508,"ancestor_record_offset":27508,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1978,"record_offset":30300,"parent_record_offset":30276,"ancestor_record_offset":27508,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1978,"record_offset":31056,"parent_record_offset":31024,"ancestor_record_offset":27508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1978,"record_offset":30324,"parent_record_offset":30300,"ancestor_record_offset":27508,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1978,"record_offset":31104,"parent_record_offset":31080,"ancestor_record_offset":27508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1978,"record_offset":31080,"parent_record_offset":31056,"ancestor_record_offset":27508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29544,"parent_record_offset":29516,"ancestor_record_offset":27508,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29596,"parent_record_offset":29572,"ancestor_record_offset":27508,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29572,"parent_record_offset":29544,"ancestor_record_offset":27508,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29492,"parent_record_offset":29464,"ancestor_record_offset":27508,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29516,"parent_record_offset":29492,"ancestor_record_offset":27508,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29688,"parent_record_offset":29516,"ancestor_record_offset":27508,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":29800,"parent_record_offset":29776,"ancestor_record_offset":27508,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":31500,"parent_record_offset":31476,"ancestor_record_offset":27508,"inlinee":10758,"source_line":1976,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1978,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":27508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":31240,"parent_record_offset":31212,"ancestor_record_offset":27508,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1978,"record_offset":27932,"parent_record_offset":27900,"ancestor_record_offset":27508,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1978,"record_offset":29360,"parent_record_offset":29316,"ancestor_record_offset":27508,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1978,"record_offset":28180,"parent_record_offset":28152,"ancestor_record_offset":27508,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1978,"record_offset":30148,"parent_record_offset":29316,"ancestor_record_offset":27508,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":29752,"parent_record_offset":29464,"ancestor_record_offset":27508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27508,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":27508,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":28828,"parent_record_offset":28800,"ancestor_record_offset":27508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":33108,"parent_record_offset":33084,"ancestor_record_offset":27508,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":29644,"parent_record_offset":29620,"ancestor_record_offset":27508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":28572,"parent_record_offset":28548,"ancestor_record_offset":27508,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":28596,"parent_record_offset":28572,"ancestor_record_offset":27508,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":28900,"parent_record_offset":28872,"ancestor_record_offset":27508,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":28548,"parent_record_offset":28432,"ancestor_record_offset":27508,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1978,"record_offset":27900,"parent_record_offset":27872,"ancestor_record_offset":27508,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1978,"record_offset":28260,"parent_record_offset":28232,"ancestor_record_offset":27508,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":28400,"parent_record_offset":28368,"ancestor_record_offset":27508,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":28368,"parent_record_offset":28332,"ancestor_record_offset":27508,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1978,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":27508,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1978,"record_offset":28332,"parent_record_offset":28308,"ancestor_record_offset":27508,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1978,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":27508,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1978,"record_offset":28872,"parent_record_offset":28332,"ancestor_record_offset":27508,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":27872,"parent_record_offset":27840,"ancestor_record_offset":27508,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":27508,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":28032,"parent_record_offset":28000,"ancestor_record_offset":27508,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":28128,"parent_record_offset":28032,"ancestor_record_offset":27508,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":28000,"parent_record_offset":27976,"ancestor_record_offset":27508,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":27812,"parent_record_offset":27776,"ancestor_record_offset":27508,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1978,"record_offset":28152,"parent_record_offset":28128,"ancestor_record_offset":27508,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":28432,"parent_record_offset":28400,"ancestor_record_offset":27508,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":28716,"parent_record_offset":28432,"ancestor_record_offset":27508,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":28688,"parent_record_offset":28432,"ancestor_record_offset":27508,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":28492,"parent_record_offset":28432,"ancestor_record_offset":27508,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":29244,"parent_record_offset":29208,"ancestor_record_offset":27508,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":31624,"parent_record_offset":31592,"ancestor_record_offset":27508,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":32140,"parent_record_offset":31880,"ancestor_record_offset":27508,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1978,"record_offset":29388,"parent_record_offset":29360,"ancestor_record_offset":27508,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":29464,"parent_record_offset":29316,"ancestor_record_offset":27508,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1978,"record_offset":29724,"parent_record_offset":29464,"ancestor_record_offset":27508,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::bucket_dispose_in_order
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":30408,"parent_record_offset":30368,"ancestor_record_offset":27508,"inlinee":39357,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1978,"record_offset":31524,"parent_record_offset":31500,"ancestor_record_offset":27508,"inlinee":87097,"source_line":118,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1978,"record_offset":29416,"parent_record_offset":29388,"ancestor_record_offset":27508,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1978,"record_offset":29316,"parent_record_offset":29280,"ancestor_record_offset":27508,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1978,"record_offset":29280,"parent_record_offset":29244,"ancestor_record_offset":27508,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$14::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B1A
// 首条位置: {"module":703,"inlinee":39706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":60960,"parent_record_offset":60936,"ancestor_record_offset":60936,"inlinee":12821,"source_line":1151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":703,"record_offset":60984,"parent_record_offset":60960,"ancestor_record_offset":60936,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":703,"record_offset":61032,"parent_record_offset":61008,"ancestor_record_offset":60936,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":703,"record_offset":61008,"parent_record_offset":60984,"ancestor_record_offset":60936,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1974,"record_offset":856,"rva":25968896,"va":5394678016}
// 编译实例 RVAs: 0x18c4100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25702272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e880d077fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394678347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20001787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e805cccefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394678118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e7ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-902844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e84439f2ff","size":5,"soft_fail":false,"target_va":5393775312,"va":5394678151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17168974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e84efa0501","size":5,"soft_fail":false,"target_va":5411847168,"va":5394678189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16863496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e808510101","size":5,"soft_fail":false,"target_va":5411541808,"va":5394678307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16806897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e8f1730001","size":5,"soft_fail":false,"target_va":5411485168,"va":5394678266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16809298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e8527d0001","size":5,"soft_fail":false,"target_va":5411487584,"va":5394678281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12854485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e82bdb3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394678080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12854204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678016,"raw_hex":"e844dc3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394678215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1974,"record_offset":1696,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1974,"record_offset":1448,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1974,"record_offset":1420,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":86918,"source_line":173,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":1024,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":1364,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":12544,"source_line":1660,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1974,"record_offset":1236,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1974,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":856,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1628,"parent_record_offset":1504,"ancestor_record_offset":856,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1088,"parent_record_offset":1052,"ancestor_record_offset":856,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":856,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":856,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1974,"record_offset":1264,"parent_record_offset":1236,"ancestor_record_offset":856,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1974,"record_offset":1296,"parent_record_offset":1264,"ancestor_record_offset":856,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1974,"record_offset":1324,"parent_record_offset":1296,"ancestor_record_offset":856,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":1476,"parent_record_offset":1448,"ancestor_record_offset":856,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":1144,"parent_record_offset":1052,"ancestor_record_offset":856,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":1540,"parent_record_offset":1504,"ancestor_record_offset":856,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":856,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":856,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1974,"record_offset":1168,"parent_record_offset":1144,"ancestor_record_offset":856,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1974,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":856,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1974,"record_offset":1728,"rva":25969248,"va":5394678368}
// 编译实例 RVAs: 0x18c4260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25702800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e870ce77fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394678875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13518660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e8bcb831ff","size":5,"soft_fail":false,"target_va":5381160176,"va":5394678831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418adf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e8f89bfeff","size":5,"soft_fail":false,"target_va":5394587536,"va":5394678675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16863273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e829500101","size":5,"soft_fail":false,"target_va":5411542064,"va":5394678786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16806418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e812720001","size":5,"soft_fail":false,"target_va":5411485168,"va":5394678745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16808819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e8737b0001","size":5,"soft_fail":false,"target_va":5411487584,"va":5394678760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12854830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e8d2d93bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394678425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12854679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394678368,"raw_hex":"e869da3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394678690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1974,"record_offset":2520,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1974,"record_offset":2272,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1974,"record_offset":2164,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":1896,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":2108,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":12544,"source_line":1660,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1974,"record_offset":2244,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1974,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1728,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1974,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":1728,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1974,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":1728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":2452,"parent_record_offset":2328,"ancestor_record_offset":1728,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1960,"parent_record_offset":1924,"ancestor_record_offset":1728,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1728,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1974,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1728,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1974,"record_offset":2212,"parent_record_offset":2164,"ancestor_record_offset":1728,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1974,"record_offset":2300,"parent_record_offset":2272,"ancestor_record_offset":1728,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":2016,"parent_record_offset":1924,"ancestor_record_offset":1728,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":2364,"parent_record_offset":2328,"ancestor_record_offset":1728,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1728,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1974,"record_offset":2328,"parent_record_offset":2300,"ancestor_record_offset":1728,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1974,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1728,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1974,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":1728,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1974,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1728,"inlinee":86866,"source_line":128,"archive":"_pdb/B/modules/01974.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":17088,"rva":25979632,"va":5394688752}
// 编译实例 RVAs: 0x18c6af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25714082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e85ea277fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394690077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24531890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e84eac89fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394690013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20012572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e8e4a1cefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394688903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e83c89feff","size":5,"soft_fail":false,"target_va":5394593792,"va":5394689727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16852331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e86b250101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394689960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e80ca5ffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394689647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16802440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e888620001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394689923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16804857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e8f96b0001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394689938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16800111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e86f590001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394688828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16801856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e840600001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394689803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12865269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e80bb13bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394688864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12865091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394688752,"raw_hex":"e8bdb13bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394689102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":18964,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":18716,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":19816,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":19976,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":17412,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":20484,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":19784,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":20032,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":17748,"parent_record_offset":17088,"ancestor_record_offset":17088,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":20000,"parent_record_offset":19976,"ancestor_record_offset":17088,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":19348,"parent_record_offset":19000,"ancestor_record_offset":17088,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":17088,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":18040,"parent_record_offset":18016,"ancestor_record_offset":17088,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":18124,"parent_record_offset":18100,"ancestor_record_offset":17088,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":18064,"parent_record_offset":18040,"ancestor_record_offset":17088,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":18540,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":17564,"parent_record_offset":17472,"ancestor_record_offset":17088,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":17088,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":18244,"parent_record_offset":18216,"ancestor_record_offset":17088,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":18388,"parent_record_offset":18364,"ancestor_record_offset":17088,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":19188,"parent_record_offset":19140,"ancestor_record_offset":17088,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":19140,"parent_record_offset":19084,"ancestor_record_offset":17088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":19056,"parent_record_offset":19000,"ancestor_record_offset":17088,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":19084,"parent_record_offset":19056,"ancestor_record_offset":17088,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":19256,"parent_record_offset":19084,"ancestor_record_offset":17088,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":19228,"parent_record_offset":19084,"ancestor_record_offset":17088,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":20180,"parent_record_offset":20116,"ancestor_record_offset":17088,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":19884,"parent_record_offset":19844,"ancestor_record_offset":17088,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":17592,"parent_record_offset":17472,"ancestor_record_offset":17088,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":18896,"parent_record_offset":18772,"ancestor_record_offset":17088,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":17508,"parent_record_offset":17472,"ancestor_record_offset":17088,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":17088,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":17088,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":17088,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":17088,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":18184,"parent_record_offset":18156,"ancestor_record_offset":17088,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":18216,"parent_record_offset":18184,"ancestor_record_offset":17088,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":18156,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":18016,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":18568,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":18100,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":17988,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":18448,"parent_record_offset":18424,"ancestor_record_offset":17088,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":17772,"parent_record_offset":17748,"ancestor_record_offset":17088,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":17796,"parent_record_offset":17772,"ancestor_record_offset":17088,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":18424,"parent_record_offset":18316,"ancestor_record_offset":17088,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":17828,"parent_record_offset":17796,"ancestor_record_offset":17088,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":18364,"parent_record_offset":18340,"ancestor_record_offset":17088,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":18340,"parent_record_offset":18316,"ancestor_record_offset":17088,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":18660,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":17880,"parent_record_offset":17856,"ancestor_record_offset":17088,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":17856,"parent_record_offset":17796,"ancestor_record_offset":17088,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":17904,"parent_record_offset":17880,"ancestor_record_offset":17088,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":17960,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":18316,"parent_record_offset":17904,"ancestor_record_offset":17088,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":17088,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":18744,"parent_record_offset":18716,"ancestor_record_offset":17088,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":19844,"parent_record_offset":19816,"ancestor_record_offset":17088,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":20116,"parent_record_offset":20088,"ancestor_record_offset":17088,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":17652,"parent_record_offset":17436,"ancestor_record_offset":17088,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":18808,"parent_record_offset":18772,"ancestor_record_offset":17088,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":17472,"parent_record_offset":17436,"ancestor_record_offset":17088,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":17436,"parent_record_offset":17412,"ancestor_record_offset":17088,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":18772,"parent_record_offset":18744,"ancestor_record_offset":17088,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":17680,"parent_record_offset":17652,"ancestor_record_offset":17088,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":18836,"parent_record_offset":18808,"ancestor_record_offset":17088,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":19000,"parent_record_offset":18964,"ancestor_record_offset":17088,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":18496,"parent_record_offset":18472,"ancestor_record_offset":17088,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":20548,"rva":25980992,"va":5394690112}
// 编译实例 RVAs: 0x18c7040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25715442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e80e9d77fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394691437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24533250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8fea689fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394691373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20013932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8949ccefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394690263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af220
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e80c7efeff","size":5,"soft_fail":false,"target_va":5394592288,"va":5394691087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16850971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e81b200101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394691320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8bc9fffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394691007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16801080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8385d0001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394691283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16803497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8a9660001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394691298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16798751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e81f540001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394690188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16800496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8f05a0001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394691163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12866629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e8bbab3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394690224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12866451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394690112,"raw_hex":"e86dac3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394690462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":22452,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":22204,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":23304,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":23464,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":20900,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":23972,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":23272,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":23520,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":21236,"parent_record_offset":20548,"ancestor_record_offset":20548,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":23488,"parent_record_offset":23464,"ancestor_record_offset":20548,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":22836,"parent_record_offset":22488,"ancestor_record_offset":20548,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":20548,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":20548,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":21612,"parent_record_offset":21588,"ancestor_record_offset":20548,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":21552,"parent_record_offset":21528,"ancestor_record_offset":20548,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":22028,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":21052,"parent_record_offset":20960,"ancestor_record_offset":20548,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":21760,"parent_record_offset":21732,"ancestor_record_offset":20548,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":21732,"parent_record_offset":21704,"ancestor_record_offset":20548,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":20548,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":22676,"parent_record_offset":22628,"ancestor_record_offset":20548,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":22628,"parent_record_offset":22572,"ancestor_record_offset":20548,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":22544,"parent_record_offset":22488,"ancestor_record_offset":20548,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":22572,"parent_record_offset":22544,"ancestor_record_offset":20548,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":22744,"parent_record_offset":22572,"ancestor_record_offset":20548,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":22716,"parent_record_offset":22572,"ancestor_record_offset":20548,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":23668,"parent_record_offset":23604,"ancestor_record_offset":20548,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":23372,"parent_record_offset":23332,"ancestor_record_offset":20548,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":21080,"parent_record_offset":20960,"ancestor_record_offset":20548,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":22384,"parent_record_offset":22260,"ancestor_record_offset":20548,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":20996,"parent_record_offset":20960,"ancestor_record_offset":20548,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":20548,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":20548,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":21020,"parent_record_offset":20996,"ancestor_record_offset":20548,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":22408,"parent_record_offset":22384,"ancestor_record_offset":20548,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":20548,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":20548,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":21644,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":21504,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":22056,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":21588,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":21476,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":20548,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":21260,"parent_record_offset":21236,"ancestor_record_offset":20548,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":21284,"parent_record_offset":21260,"ancestor_record_offset":20548,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":21912,"parent_record_offset":21804,"ancestor_record_offset":20548,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":21316,"parent_record_offset":21284,"ancestor_record_offset":20548,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21852,"parent_record_offset":21828,"ancestor_record_offset":20548,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21828,"parent_record_offset":21804,"ancestor_record_offset":20548,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":22148,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21368,"parent_record_offset":21344,"ancestor_record_offset":20548,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21344,"parent_record_offset":21284,"ancestor_record_offset":20548,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21392,"parent_record_offset":21368,"ancestor_record_offset":20548,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21448,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":21804,"parent_record_offset":21392,"ancestor_record_offset":20548,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":23576,"parent_record_offset":23548,"ancestor_record_offset":20548,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":22232,"parent_record_offset":22204,"ancestor_record_offset":20548,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":23332,"parent_record_offset":23304,"ancestor_record_offset":20548,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":20548,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":21140,"parent_record_offset":20924,"ancestor_record_offset":20548,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":22296,"parent_record_offset":22260,"ancestor_record_offset":20548,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":20960,"parent_record_offset":20924,"ancestor_record_offset":20548,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":20548,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":22260,"parent_record_offset":22232,"ancestor_record_offset":20548,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":21168,"parent_record_offset":21140,"ancestor_record_offset":20548,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":22324,"parent_record_offset":22296,"ancestor_record_offset":20548,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":22488,"parent_record_offset":22452,"ancestor_record_offset":20548,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":21984,"parent_record_offset":21960,"ancestor_record_offset":20548,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":24036,"rva":25982352,"va":5394691472}
// 编译实例 RVAs: 0x18c7590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25716802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8be9777fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394692797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24534610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8aea189fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394692733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20015292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e84497cefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394691623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418afa20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8bc80feff","size":5,"soft_fail":false,"target_va":5394594336,"va":5394692447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16849611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8cb1a0101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394692680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e86c9affff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394692367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16799720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8e8570001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394692643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16802137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e859610001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394692658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16797391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8cf4e0001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394691548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16799136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e8a0550001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394692523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12867989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e86ba63bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394691584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12867811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394691472,"raw_hex":"e81da73bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394691822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":25924,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":25676,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":26776,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":26936,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":24372,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":27444,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":26744,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":26992,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":24708,"parent_record_offset":24036,"ancestor_record_offset":24036,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":24036,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":26308,"parent_record_offset":25960,"ancestor_record_offset":24036,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":25432,"parent_record_offset":25408,"ancestor_record_offset":24036,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":25000,"parent_record_offset":24976,"ancestor_record_offset":24036,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":25084,"parent_record_offset":25060,"ancestor_record_offset":24036,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":25024,"parent_record_offset":25000,"ancestor_record_offset":24036,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":25500,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":24524,"parent_record_offset":24432,"ancestor_record_offset":24036,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":25232,"parent_record_offset":25204,"ancestor_record_offset":24036,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":25204,"parent_record_offset":25176,"ancestor_record_offset":24036,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":25348,"parent_record_offset":25324,"ancestor_record_offset":24036,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":26148,"parent_record_offset":26100,"ancestor_record_offset":24036,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":26100,"parent_record_offset":26044,"ancestor_record_offset":24036,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":26016,"parent_record_offset":25960,"ancestor_record_offset":24036,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":26044,"parent_record_offset":26016,"ancestor_record_offset":24036,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":26216,"parent_record_offset":26044,"ancestor_record_offset":24036,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":26188,"parent_record_offset":26044,"ancestor_record_offset":24036,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":27140,"parent_record_offset":27076,"ancestor_record_offset":24036,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":26844,"parent_record_offset":26804,"ancestor_record_offset":24036,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":24552,"parent_record_offset":24432,"ancestor_record_offset":24036,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":25856,"parent_record_offset":25732,"ancestor_record_offset":24036,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":24468,"parent_record_offset":24432,"ancestor_record_offset":24036,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":27164,"parent_record_offset":27140,"ancestor_record_offset":24036,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":24036,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":24036,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":24036,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":25144,"parent_record_offset":25116,"ancestor_record_offset":24036,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":25176,"parent_record_offset":25144,"ancestor_record_offset":24036,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":25116,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":24976,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":25528,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":25060,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":24948,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":25408,"parent_record_offset":25384,"ancestor_record_offset":24036,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24036,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":24036,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":25384,"parent_record_offset":25276,"ancestor_record_offset":24036,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":24788,"parent_record_offset":24756,"ancestor_record_offset":24036,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":25324,"parent_record_offset":25300,"ancestor_record_offset":24036,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":25300,"parent_record_offset":25276,"ancestor_record_offset":24036,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":25620,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":24840,"parent_record_offset":24816,"ancestor_record_offset":24036,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":24816,"parent_record_offset":24756,"ancestor_record_offset":24036,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":24864,"parent_record_offset":24840,"ancestor_record_offset":24036,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":24920,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":25276,"parent_record_offset":24864,"ancestor_record_offset":24036,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":27048,"parent_record_offset":27020,"ancestor_record_offset":24036,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":25704,"parent_record_offset":25676,"ancestor_record_offset":24036,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":26804,"parent_record_offset":26776,"ancestor_record_offset":24036,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":27076,"parent_record_offset":27048,"ancestor_record_offset":24036,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":24612,"parent_record_offset":24396,"ancestor_record_offset":24036,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":25768,"parent_record_offset":25732,"ancestor_record_offset":24036,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":24432,"parent_record_offset":24396,"ancestor_record_offset":24036,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":24036,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":25732,"parent_record_offset":25704,"ancestor_record_offset":24036,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":24640,"parent_record_offset":24612,"ancestor_record_offset":24036,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":25796,"parent_record_offset":25768,"ancestor_record_offset":24036,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":25960,"parent_record_offset":25924,"ancestor_record_offset":24036,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":25456,"parent_record_offset":25432,"ancestor_record_offset":24036,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":10152,"rva":25976912,"va":5394686032}
// 编译实例 RVAs: 0x18c6050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25711362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8feac77fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394687357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24529170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8eeb689fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394687293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20009852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e884accefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394686183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af690
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-93588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e86c92feff","size":5,"soft_fail":false,"target_va":5394593424,"va":5394687007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16855051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e80b300101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394687240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8acafffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394686927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16805160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8286d0001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394687203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16807577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e899760001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394687218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16802831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e80f640001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394686108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16804576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8e06a0001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394687083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12862549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e8abbb3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394686144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12862371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394686032,"raw_hex":"e85dbc3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394686382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":12040,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":11792,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":12892,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":13052,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":10488,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":13560,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":12860,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":13108,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":10824,"parent_record_offset":10152,"ancestor_record_offset":10152,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":10152,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":12424,"parent_record_offset":12076,"ancestor_record_offset":10152,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":10152,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":10152,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10152,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":10152,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":11616,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":10640,"parent_record_offset":10548,"ancestor_record_offset":10152,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":11348,"parent_record_offset":11320,"ancestor_record_offset":10152,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":11320,"parent_record_offset":11292,"ancestor_record_offset":10152,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10152,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":12264,"parent_record_offset":12216,"ancestor_record_offset":10152,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":12216,"parent_record_offset":12160,"ancestor_record_offset":10152,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":12132,"parent_record_offset":12076,"ancestor_record_offset":10152,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":12160,"parent_record_offset":12132,"ancestor_record_offset":10152,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":12332,"parent_record_offset":12160,"ancestor_record_offset":10152,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":12304,"parent_record_offset":12160,"ancestor_record_offset":10152,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":13256,"parent_record_offset":13192,"ancestor_record_offset":10152,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":12960,"parent_record_offset":12920,"ancestor_record_offset":10152,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":10668,"parent_record_offset":10548,"ancestor_record_offset":10152,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":11972,"parent_record_offset":11848,"ancestor_record_offset":10152,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":10584,"parent_record_offset":10548,"ancestor_record_offset":10152,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":10152,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":10152,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":10152,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":10152,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":11260,"parent_record_offset":11232,"ancestor_record_offset":10152,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":11292,"parent_record_offset":11260,"ancestor_record_offset":10152,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":11232,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":11092,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":11644,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":11176,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":11064,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":11524,"parent_record_offset":11500,"ancestor_record_offset":10152,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":10152,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":10152,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":11500,"parent_record_offset":11392,"ancestor_record_offset":10152,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":10904,"parent_record_offset":10872,"ancestor_record_offset":10152,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10152,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":10152,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":11736,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":10152,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":10932,"parent_record_offset":10872,"ancestor_record_offset":10152,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":10152,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":11036,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":11392,"parent_record_offset":10980,"ancestor_record_offset":10152,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":10152,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":11820,"parent_record_offset":11792,"ancestor_record_offset":10152,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":12920,"parent_record_offset":12892,"ancestor_record_offset":10152,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":13192,"parent_record_offset":13164,"ancestor_record_offset":10152,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":10728,"parent_record_offset":10512,"ancestor_record_offset":10152,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":11884,"parent_record_offset":11848,"ancestor_record_offset":10152,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":10548,"parent_record_offset":10512,"ancestor_record_offset":10152,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":10152,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":10152,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":10756,"parent_record_offset":10728,"ancestor_record_offset":10152,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":11912,"parent_record_offset":11884,"ancestor_record_offset":10152,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":12076,"parent_record_offset":12040,"ancestor_record_offset":10152,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":10152,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":6688,"rva":25975552,"va":5394684672}
// 编译实例 RVAs: 0x18c5b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25710002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e84eb277fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394685997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24527810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e83ebc89fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394685933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20008492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8d4b1cefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394684823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8bc94feff","size":5,"soft_fail":false,"target_va":5394592656,"va":5394685647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16856411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e85b350101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394685880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8fcb4ffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394685567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16806520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e878720001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394685843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16808937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8e97b0001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394685858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16804191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e85f690001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394684748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16805936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e830700001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394685723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12861189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8fbc03bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394684784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12861011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394684672,"raw_hex":"e8adc13bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394685022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":8568,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":8320,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":9420,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":9580,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":7016,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":10088,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":9388,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":9636,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":7352,"parent_record_offset":6688,"ancestor_record_offset":6688,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":6688,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":8952,"parent_record_offset":8604,"ancestor_record_offset":6688,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":6688,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":6688,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":6688,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":6688,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":8144,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":7168,"parent_record_offset":7076,"ancestor_record_offset":6688,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":7876,"parent_record_offset":7848,"ancestor_record_offset":6688,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":6688,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":6688,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":8792,"parent_record_offset":8744,"ancestor_record_offset":6688,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":8744,"parent_record_offset":8688,"ancestor_record_offset":6688,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":8660,"parent_record_offset":8604,"ancestor_record_offset":6688,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":6688,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":8860,"parent_record_offset":8688,"ancestor_record_offset":6688,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":8832,"parent_record_offset":8688,"ancestor_record_offset":6688,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":9784,"parent_record_offset":9720,"ancestor_record_offset":6688,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":9488,"parent_record_offset":9448,"ancestor_record_offset":6688,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":7196,"parent_record_offset":7076,"ancestor_record_offset":6688,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":8500,"parent_record_offset":8376,"ancestor_record_offset":6688,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":7112,"parent_record_offset":7076,"ancestor_record_offset":6688,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":6688,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6688,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":6688,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":6688,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":6688,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":6688,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":7760,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":7620,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":8172,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":7704,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":7592,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":6688,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":6688,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":6688,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":8028,"parent_record_offset":7920,"ancestor_record_offset":6688,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":7432,"parent_record_offset":7400,"ancestor_record_offset":6688,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":6688,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":6688,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":8264,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":6688,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7460,"parent_record_offset":7400,"ancestor_record_offset":6688,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":6688,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7564,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":7920,"parent_record_offset":7508,"ancestor_record_offset":6688,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":9692,"parent_record_offset":9664,"ancestor_record_offset":6688,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":6688,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":9448,"parent_record_offset":9420,"ancestor_record_offset":6688,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":9720,"parent_record_offset":9692,"ancestor_record_offset":6688,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":7256,"parent_record_offset":7040,"ancestor_record_offset":6688,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":8412,"parent_record_offset":8376,"ancestor_record_offset":6688,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":7076,"parent_record_offset":7040,"ancestor_record_offset":6688,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6688,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":6688,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":6688,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":6688,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":8604,"parent_record_offset":8568,"ancestor_record_offset":6688,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":6688,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":3220,"rva":25974192,"va":5394683312}
// 编译实例 RVAs: 0x18c55b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25708642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e89eb777fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394684637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24526450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e88ec189fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394684573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20007132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e824b7cefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394683463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af500
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e87c9bfeff","size":5,"soft_fail":false,"target_va":5394593024,"va":5394684287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16857771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e8ab3a0101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394684520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e84cbaffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394684207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16807880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e8c8770001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394684483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16810297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e839810001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394684498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16805551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e8af6e0001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394683388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16807296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e880750001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394684363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12859829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e84bc63bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394683424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12859651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394683312,"raw_hex":"e8fdc63bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394683662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":5104,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":4856,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":5956,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":6116,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":3552,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":6624,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":5924,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":6172,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":3888,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":3220,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":5488,"parent_record_offset":5140,"ancestor_record_offset":3220,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":3220,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3220,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3220,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3220,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":4680,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":3704,"parent_record_offset":3612,"ancestor_record_offset":3220,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":3220,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":4384,"parent_record_offset":4356,"ancestor_record_offset":3220,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3220,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":5328,"parent_record_offset":5280,"ancestor_record_offset":3220,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":5280,"parent_record_offset":5224,"ancestor_record_offset":3220,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":5196,"parent_record_offset":5140,"ancestor_record_offset":3220,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":3220,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":5396,"parent_record_offset":5224,"ancestor_record_offset":3220,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":5368,"parent_record_offset":5224,"ancestor_record_offset":3220,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":6320,"parent_record_offset":6256,"ancestor_record_offset":3220,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":6024,"parent_record_offset":5984,"ancestor_record_offset":3220,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":3732,"parent_record_offset":3612,"ancestor_record_offset":3220,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":5036,"parent_record_offset":4912,"ancestor_record_offset":3220,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":3648,"parent_record_offset":3612,"ancestor_record_offset":3220,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":3220,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":3220,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":3220,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":3220,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":3220,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":4356,"parent_record_offset":4324,"ancestor_record_offset":3220,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":4296,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":4156,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":4708,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":4240,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":4128,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3220,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3220,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":3220,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":4564,"parent_record_offset":4456,"ancestor_record_offset":3220,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":3220,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3220,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3220,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4800,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3220,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":3996,"parent_record_offset":3936,"ancestor_record_offset":3220,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":3220,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4100,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":4456,"parent_record_offset":4044,"ancestor_record_offset":3220,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":6228,"parent_record_offset":6200,"ancestor_record_offset":3220,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":4884,"parent_record_offset":4856,"ancestor_record_offset":3220,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":3220,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":3220,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":3792,"parent_record_offset":3576,"ancestor_record_offset":3220,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":4948,"parent_record_offset":4912,"ancestor_record_offset":3220,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":3612,"parent_record_offset":3576,"ancestor_record_offset":3220,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":3220,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":3220,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":3220,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":3220,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":5140,"parent_record_offset":5104,"ancestor_record_offset":3220,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3220,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1978,"record_offset":13624,"rva":25978272,"va":5394687392}
// 编译实例 RVAs: 0x18c65a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25712722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e8aea777fe","size":5,"soft_fail":false,"target_va":5368976000,"va":5394688717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24530530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e89eb189fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394688653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20011212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e834a7cefe","size":5,"soft_fail":false,"target_va":5374676336,"va":5394687543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418af090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-96484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e81c87feff","size":5,"soft_fail":false,"target_va":5394591888,"va":5394688367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16853691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e8bb2a0101","size":5,"soft_fail":false,"target_va":5411542296,"va":5394688600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e85caaffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394688287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16803800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e8d8670001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394688563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16806217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e849710001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394688578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16801471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e8bf5e0001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394687468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16803216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e890650001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394688443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12863909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e85bb63bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394687504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12863731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394687392,"raw_hex":"e80db73bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394687742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1978,"record_offset":15504,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1978,"record_offset":15256,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":16356,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":16516,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":13952,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_immediate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":17024,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":39331,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::dispose_resource_to_cb_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":16324,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":39330,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1978,"record_offset":16572,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":87093,"source_line":200,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1978,"record_offset":14288,"parent_record_offset":13624,"ancestor_record_offset":13624,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1978,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":13624,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1978,"record_offset":15888,"parent_record_offset":15540,"ancestor_record_offset":13624,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1978,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":13624,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":13624,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1978,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":13624,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":13624,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1978,"record_offset":15080,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1978,"record_offset":14104,"parent_record_offset":14012,"ancestor_record_offset":13624,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":14812,"parent_record_offset":14784,"ancestor_record_offset":13624,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1978,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":13624,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1978,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":13624,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1978,"record_offset":15728,"parent_record_offset":15680,"ancestor_record_offset":13624,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1978,"record_offset":15680,"parent_record_offset":15624,"ancestor_record_offset":13624,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1978,"record_offset":15596,"parent_record_offset":15540,"ancestor_record_offset":13624,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":15624,"parent_record_offset":15596,"ancestor_record_offset":13624,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":15796,"parent_record_offset":15624,"ancestor_record_offset":13624,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1978,"record_offset":15768,"parent_record_offset":15624,"ancestor_record_offset":13624,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":16720,"parent_record_offset":16656,"ancestor_record_offset":13624,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":16424,"parent_record_offset":16384,"ancestor_record_offset":13624,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":14132,"parent_record_offset":14012,"ancestor_record_offset":13624,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":15436,"parent_record_offset":15312,"ancestor_record_offset":13624,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":14048,"parent_record_offset":14012,"ancestor_record_offset":13624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":16744,"parent_record_offset":16720,"ancestor_record_offset":13624,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":13624,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":14072,"parent_record_offset":14048,"ancestor_record_offset":13624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1978,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":13624,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":14724,"parent_record_offset":14696,"ancestor_record_offset":13624,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":14756,"parent_record_offset":14724,"ancestor_record_offset":13624,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1978,"record_offset":14696,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":14556,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":15108,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1978,"record_offset":14640,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1978,"record_offset":14528,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1978,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":13624,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":13624,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13624,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":14964,"parent_record_offset":14856,"ancestor_record_offset":13624,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1978,"record_offset":14368,"parent_record_offset":14336,"ancestor_record_offset":13624,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":13624,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":13624,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":15200,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13624,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14396,"parent_record_offset":14336,"ancestor_record_offset":13624,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":13624,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14500,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1978,"record_offset":14856,"parent_record_offset":14444,"ancestor_record_offset":13624,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1978,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":13624,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1978,"record_offset":15284,"parent_record_offset":15256,"ancestor_record_offset":13624,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":13624,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1978,"record_offset":16656,"parent_record_offset":16628,"ancestor_record_offset":13624,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":14192,"parent_record_offset":13976,"ancestor_record_offset":13624,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":15348,"parent_record_offset":15312,"ancestor_record_offset":13624,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":14012,"parent_record_offset":13976,"ancestor_record_offset":13624,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13624,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1978,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":13624,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":14220,"parent_record_offset":14192,"ancestor_record_offset":13624,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1978,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":13624,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/leek.rs
//     evidence={"module":1978,"record_offset":15540,"parent_record_offset":15504,"ancestor_record_offset":13624,"inlinee":39320,"source_line":81,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1978,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":13624,"inlinee":78019,"source_line":31,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$15::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B00
// 首条位置: {"module":703,"inlinee":39680}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":57988,"parent_record_offset":57964,"ancestor_record_offset":57964,"inlinee":4594,"source_line":153,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":58012,"parent_record_offset":57988,"ancestor_record_offset":57964,"inlinee":4596,"source_line":558,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$17::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15433
// 首条位置: {"module":1978,"inlinee":87091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$19::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15439
// 首条位置: {"module":1978,"inlinee":87097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$21::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15352
// 首条位置: {"module":1962,"inlinee":86866}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::DisposalBucket::dispose
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x994F
// 首条位置: {"module":684,"inlinee":39247}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$10::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":6224,"inlinee":86304,"source_line":328,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$11::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":86299,"source_line":334,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$12::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":9784,"inlinee":86316,"source_line":340,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$13::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":4848,"inlinee":39253,"source_line":346,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$4::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":4612,"inlinee":39251,"source_line":292,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$5::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":3624,"parent_record_offset":3596,"ancestor_record_offset":3596,"inlinee":39249,"source_line":298,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$6::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1921,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":504,"inlinee":86328,"source_line":304,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":29316,"parent_record_offset":29292,"ancestor_record_offset":29292,"inlinee":86797,"source_line":310,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":30056,"parent_record_offset":30024,"ancestor_record_offset":30024,"inlinee":86799,"source_line":316,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$9::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1920,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":7728,"inlinee":86310,"source_line":322,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3596,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":3596,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3596,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3692,"parent_record_offset":3652,"ancestor_record_offset":3596,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3652,"parent_record_offset":3624,"ancestor_record_offset":3596,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":3804,"parent_record_offset":3652,"ancestor_record_offset":3596,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket
// 记录形式: compiled_procedure; instances=1
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":73540,"rva":12554992,"va":5381264112}
// 编译实例 RVAs: 0xbf92f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1377651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e873051500","size":5,"soft_fail":false,"target_va":5382644528,"va":5381266872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11116934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e87a5e56ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381275057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6193783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8897da1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381275282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5750720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e84040a8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381266747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879c00
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3677198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8f2e3c7ff","size":5,"soft_fail":false,"target_va":5377596416,"va":5381273609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be1cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-103251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8ad6cfeff","size":5,"soft_fail":false,"target_va":5381168368,"va":5381271614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=21
//     evidence={"assembly":"\tcallq\t30269667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8e3e0cd01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381271912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30270533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e845e4cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381272342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-47868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e80445ffff","size":5,"soft_fail":false,"target_va":5381216320,"va":5381264183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t30279102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8be05ce01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381264253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13401077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8f57bcc00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381265286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30220437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e89520cd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381271926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30222854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8062acd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381271941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t552220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e81c6d0800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381271375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t552382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8be6d0800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381271629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t120807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8e7d70100","size":5,"soft_fail":false,"target_va":5381385536,"va":5381264724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=22
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t125996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e82cec0100","size":5,"soft_fail":false,"target_va":5381390640,"va":5381264639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t121093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e805d90100","size":5,"soft_fail":false,"target_va":5381386480,"va":5381265382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t120545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8e1d60100","size":5,"soft_fail":false,"target_va":5381384704,"va":5381264154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::free
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t78134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e836310100","size":5,"soft_fail":false,"target_va":5381349712,"va":5381271573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e8bbebffff","size":5,"soft_fail":false,"target_va":5381261696,"va":5381266880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=22
//     evidence={"assembly":"\tcallq\t30472735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e81ffad001","size":5,"soft_fail":false,"target_va":5411737808,"va":5381265068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t13400351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e81f79cc00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381272796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=21
//     evidence={"assembly":"\tcallq\t30265199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381264112,"raw_hex":"e86fcfcd01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381271900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t*448(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381264112,"raw_hex":"ff97c0010000","size":6,"soft_fail":false,"va":5381265230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":110696,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":78244,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":74268,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":703,"record_offset":78448,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":85752,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39712,"source_line":1242,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":107536,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39719,"source_line":1298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_framebuffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":96868,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39716,"source_line":1200,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":82196,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39711,"source_line":1148,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_image_view
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":78640,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39710,"source_line":1172,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":100424,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39717,"source_line":1186,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_sampler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":103980,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39718,"source_line":1000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":93312,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39715,"source_line":1284,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::destroy_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":703,"record_offset":89788,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39714,"source_line":31,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":87012,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":703,"record_offset":89524,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":78700,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":111492,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":110724,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":87044,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":86928,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":75276,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":73812,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":74300,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_buffers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":85780,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39335,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_descriptor_pools
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":107564,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39340,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_framebuffers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":96896,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39337,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_image_views
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":78668,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39333,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_images
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":82224,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39334,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_render_passes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":100452,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39338,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_samplers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":104008,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39339,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_semaphores
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":93340,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39336,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::destroy_swapchain
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":89748,"parent_record_offset":73540,"ancestor_record_offset":73540,"inlinee":39341,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":111728,"parent_record_offset":111704,"ancestor_record_offset":73540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":111704,"parent_record_offset":111648,"ancestor_record_offset":73540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":78300,"parent_record_offset":78276,"ancestor_record_offset":73540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":111676,"parent_record_offset":111648,"ancestor_record_offset":73540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":111648,"parent_record_offset":111624,"ancestor_record_offset":73540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":78348,"parent_record_offset":78324,"ancestor_record_offset":73540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":78324,"parent_record_offset":78300,"ancestor_record_offset":73540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":111624,"parent_record_offset":111600,"ancestor_record_offset":73540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":78276,"parent_record_offset":78244,"ancestor_record_offset":73540,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":78388,"parent_record_offset":78244,"ancestor_record_offset":73540,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":78416,"parent_record_offset":78244,"ancestor_record_offset":73540,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":703,"record_offset":78576,"parent_record_offset":78552,"ancestor_record_offset":73540,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75612,"parent_record_offset":75584,"ancestor_record_offset":73540,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75664,"parent_record_offset":75640,"ancestor_record_offset":73540,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75640,"parent_record_offset":75612,"ancestor_record_offset":73540,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75560,"parent_record_offset":75532,"ancestor_record_offset":73540,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75584,"parent_record_offset":75560,"ancestor_record_offset":73540,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75756,"parent_record_offset":75584,"ancestor_record_offset":73540,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":75868,"parent_record_offset":75844,"ancestor_record_offset":73540,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":75688,"parent_record_offset":75664,"ancestor_record_offset":73540,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":703,"record_offset":73968,"parent_record_offset":73936,"ancestor_record_offset":73540,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":75428,"parent_record_offset":75384,"ancestor_record_offset":73540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":78552,"parent_record_offset":78448,"ancestor_record_offset":73540,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":87100,"parent_record_offset":87068,"ancestor_record_offset":73540,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":87128,"parent_record_offset":87068,"ancestor_record_offset":73540,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":74216,"parent_record_offset":74188,"ancestor_record_offset":73540,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":76216,"parent_record_offset":75384,"ancestor_record_offset":73540,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=114
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":75820,"parent_record_offset":75532,"ancestor_record_offset":73540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":89484,"parent_record_offset":89460,"ancestor_record_offset":73540,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":111364,"parent_record_offset":111340,"ancestor_record_offset":73540,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":74712,"parent_record_offset":74688,"ancestor_record_offset":73540,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":74688,"parent_record_offset":74664,"ancestor_record_offset":73540,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":74896,"parent_record_offset":74868,"ancestor_record_offset":73540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":78600,"parent_record_offset":78576,"ancestor_record_offset":73540,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":75712,"parent_record_offset":75688,"ancestor_record_offset":73540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":89436,"parent_record_offset":89232,"ancestor_record_offset":73540,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":89460,"parent_record_offset":89436,"ancestor_record_offset":73540,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":74640,"parent_record_offset":74616,"ancestor_record_offset":73540,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":111340,"parent_record_offset":111316,"ancestor_record_offset":73540,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":74664,"parent_record_offset":74640,"ancestor_record_offset":73540,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":74968,"parent_record_offset":74940,"ancestor_record_offset":73540,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":78476,"parent_record_offset":78448,"ancestor_record_offset":73540,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":74616,"parent_record_offset":74500,"ancestor_record_offset":73540,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":111316,"parent_record_offset":111228,"ancestor_record_offset":73540,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":73936,"parent_record_offset":73908,"ancestor_record_offset":73540,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":74328,"parent_record_offset":74300,"ancestor_record_offset":73540,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":74468,"parent_record_offset":74436,"ancestor_record_offset":73540,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":74436,"parent_record_offset":74400,"ancestor_record_offset":73540,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":74376,"parent_record_offset":74352,"ancestor_record_offset":73540,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":74400,"parent_record_offset":74376,"ancestor_record_offset":73540,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":74352,"parent_record_offset":74328,"ancestor_record_offset":73540,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":74940,"parent_record_offset":74400,"ancestor_record_offset":73540,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":73908,"parent_record_offset":73876,"ancestor_record_offset":73540,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":73876,"parent_record_offset":73848,"ancestor_record_offset":73540,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":74068,"parent_record_offset":74036,"ancestor_record_offset":73540,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":74164,"parent_record_offset":74068,"ancestor_record_offset":73540,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":74036,"parent_record_offset":74012,"ancestor_record_offset":73540,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":73848,"parent_record_offset":73812,"ancestor_record_offset":73540,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":74188,"parent_record_offset":74164,"ancestor_record_offset":73540,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":74500,"parent_record_offset":74468,"ancestor_record_offset":73540,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":74784,"parent_record_offset":74500,"ancestor_record_offset":73540,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":74756,"parent_record_offset":74500,"ancestor_record_offset":73540,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":74560,"parent_record_offset":74500,"ancestor_record_offset":73540,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":111908,"parent_record_offset":111844,"ancestor_record_offset":73540,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":110788,"parent_record_offset":110752,"ancestor_record_offset":73540,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":86952,"parent_record_offset":86928,"ancestor_record_offset":73540,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":111932,"parent_record_offset":111908,"ancestor_record_offset":73540,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":110812,"parent_record_offset":110788,"ancestor_record_offset":73540,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":86976,"parent_record_offset":86952,"ancestor_record_offset":73540,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":111816,"parent_record_offset":111788,"ancestor_record_offset":73540,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":87156,"parent_record_offset":87068,"ancestor_record_offset":73540,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":87068,"parent_record_offset":87044,"ancestor_record_offset":73540,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":87180,"parent_record_offset":87156,"ancestor_record_offset":73540,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":110752,"parent_record_offset":110724,"ancestor_record_offset":73540,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":111844,"parent_record_offset":111816,"ancestor_record_offset":73540,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":110844,"parent_record_offset":110752,"ancestor_record_offset":73540,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":111880,"parent_record_offset":111844,"ancestor_record_offset":73540,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=33
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":75312,"parent_record_offset":75276,"ancestor_record_offset":73540,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":78772,"parent_record_offset":78736,"ancestor_record_offset":73540,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":79340,"parent_record_offset":79068,"ancestor_record_offset":73540,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=22
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":703,"record_offset":75456,"parent_record_offset":75428,"ancestor_record_offset":73540,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":75532,"parent_record_offset":75384,"ancestor_record_offset":73540,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":75792,"parent_record_offset":75532,"ancestor_record_offset":73540,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":703,"record_offset":75484,"parent_record_offset":75456,"ancestor_record_offset":73540,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":75384,"parent_record_offset":75348,"ancestor_record_offset":73540,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":75348,"parent_record_offset":75312,"ancestor_record_offset":73540,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$4::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9953
// 首条位置: {"module":684,"inlinee":39251}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4636,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4636,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4636,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":4696,"parent_record_offset":4660,"ancestor_record_offset":4636,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":4808,"parent_record_offset":4660,"ancestor_record_offset":4636,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$5::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9951
// 首条位置: {"module":684,"inlinee":39249}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3652,"parent_record_offset":3624,"ancestor_record_offset":3624,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3624,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":3624,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3624,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3692,"parent_record_offset":3652,"ancestor_record_offset":3624,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":3804,"parent_record_offset":3652,"ancestor_record_offset":3624,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$6::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15138
// 首条位置: {"module":1921,"inlinee":86328}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1921,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":528,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":528,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":528,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1921,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":528,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1921,"record_offset":588,"parent_record_offset":552,"ancestor_record_offset":528,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1921,"record_offset":700,"parent_record_offset":552,"ancestor_record_offset":528,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01921.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$7::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1530D
// 首条位置: {"module":1978,"inlinee":86797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29340,"parent_record_offset":29316,"ancestor_record_offset":29316,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":29316,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29448,"parent_record_offset":29424,"ancestor_record_offset":29316,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":29316,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29376,"parent_record_offset":29340,"ancestor_record_offset":29316,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":29488,"parent_record_offset":29340,"ancestor_record_offset":29316,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$8::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1530F
// 首条位置: {"module":1978,"inlinee":86799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":30088,"parent_record_offset":30056,"ancestor_record_offset":30056,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":30156,"parent_record_offset":30132,"ancestor_record_offset":30056,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":30204,"parent_record_offset":30180,"ancestor_record_offset":30056,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":30056,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":30132,"parent_record_offset":30088,"ancestor_record_offset":30056,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":30244,"parent_record_offset":30088,"ancestor_record_offset":30056,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$9::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15126
// 首条位置: {"module":1920,"inlinee":86310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":7752,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":7752,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7752,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7752,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":7812,"parent_record_offset":7776,"ancestor_record_offset":7752,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1920,"record_offset":7924,"parent_record_offset":7776,"ancestor_record_offset":7752,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$10::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15120
// 首条位置: {"module":1920,"inlinee":86304}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":6248,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":6248,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":6248,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":6248,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":6308,"parent_record_offset":6272,"ancestor_record_offset":6248,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1920,"record_offset":6420,"parent_record_offset":6272,"ancestor_record_offset":6248,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$11::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 334
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1511B
// 首条位置: {"module":1920,"inlinee":86299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":5428,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5428,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":5428,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":5428,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":5488,"parent_record_offset":5452,"ancestor_record_offset":5428,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1920,"record_offset":5600,"parent_record_offset":5452,"ancestor_record_offset":5428,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$12::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1512C
// 首条位置: {"module":1920,"inlinee":86316}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9808,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":9808,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":9808,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1920,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":9808,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1920,"record_offset":9868,"parent_record_offset":9832,"ancestor_record_offset":9808,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1920,"record_offset":9980,"parent_record_offset":9832,"ancestor_record_offset":9808,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01920.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$13::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9955
// 首条位置: {"module":684,"inlinee":39253}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":4904,"parent_record_offset":4876,"ancestor_record_offset":4876,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":4984,"parent_record_offset":4952,"ancestor_record_offset":4876,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4876,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4876,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":4952,"parent_record_offset":4904,"ancestor_record_offset":4876,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":5072,"parent_record_offset":4904,"ancestor_record_offset":4876,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
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
// 完整结构及原始类型记录: disposal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::CommandBufferDisposalBuckets
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::DisposalBucket
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::disposal::DisposalBucket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::disposal::DisposalBucket>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::disposal::DisposalBucket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >", "constructor": "lock_api::rwlock::RwLockWriteGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::disposal::DisposalBucket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >", "constructor": "lock_api::rwlock::RwLockReadGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::SequentialDisposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>", "constructor": "rhapsody::raw::disposal::SequentialDisposal::dispose_in_order", "arguments": ["rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::SequentialDisposalItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem>", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::disposal::SequentialDisposalItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> >", "constructor": "core::option::Option", "arguments": ["ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > >", "constructor": "enum2$", "arguments": ["core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panicking::assert_failed<enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > >,enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > > >", "constructor": "core::panicking::assert_failed", "arguments": ["enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > >", "enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > > >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<core::option::Option<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::resize_with<rhapsody::raw::disposal::SequentialDisposalItem,alloc::alloc::Global,rhapsody::raw::disposal::SequentialDisposalItem (*)()>", "constructor": "alloc::collections::vec_deque::VecDeque::resize_with", "arguments": ["rhapsody::raw::disposal::SequentialDisposalItem", "alloc::alloc::Global", "rhapsody::raw::disposal::SequentialDisposalItem (*)()"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::disposal::SequentialDisposalItem,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::raw::disposal::SequentialDisposalItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: disposal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: disposal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket
// 信息 ID: 47f477a0ed0e6c8cd5470b87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste @ 0x18c1014 -> 0xbf92f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x18c7e13 -> 0xbf92f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x18c8008 -> 0xbf92f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >
// 信息 ID: 309433d5db5e55d2ada0520e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >
// 信息 ID: e3b2fff91aef50380ac6755f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >
// 信息 ID: 8ee8b53dd57745b7f1b199d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >
// 信息 ID: 8cbd3d83f8600e003bd9b7a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::render_pass::ManagedRenderPass"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >
// 信息 ID: 15191b85c108c11a25f6f670
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::sampler::ManagedSampler"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >
// 信息 ID: 92a8e439912667e9f272a44b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::swapchain::ManagedSwapchain"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >
// 信息 ID: d29559232f9202767c6b11e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::texture::ManagedTexture"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste @ 0x18c0ea8 -> 0x18c65a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert
// 信息 ID: 2ddf8d09e06a5b2000f2eb27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::command_buffer::CommandBuffer::begin @ 0xbed33b -> 0x18c4260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
// 信息 ID: 1fc3dce4bae5ff216b770abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x18c7e07 -> 0x18c4100 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x18c7ffd -> 0x18c4100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::DisposalBucket::dispose
// 信息 ID: 5929d889c557e38e160271c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=3596; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=4612; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=4848; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=5140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1920, inline_record=5404; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1920, inline_record=6224; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1920, inline_record=7728; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1920, inline_record=9784; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>
// 信息 ID: 3a489fb2d9850a56a17ecac1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order -> ["rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::Queue::reclaim_command_buffer_resources @ 0xbf17d2 -> 0x18c7ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$10::dispose_to_bucket
// 信息 ID: 9c5b8eef89b7852eebd35933
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1920, inline_record=6248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$11::dispose_to_bucket
// 信息 ID: 032ac04971c61235afb6374e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1920, inline_record=5428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$12::dispose_to_bucket
// 信息 ID: 5029dc7208ae3e8cee0d5f57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1920, inline_record=9808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$13::dispose_to_bucket
// 信息 ID: fddac0f9f12cbf3a5e5cf1e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=4876; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::buffer::impl$0::dispose; module=1921, inline_record=768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$14::default
// 信息 ID: 0450bd9e7fd77d84c9dd542c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=60936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$15::default
// 信息 ID: 4cbaf4c276a1a810cbbd82a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=57964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$17::fmt
// 信息 ID: f43b147e49fcf8f71c370b29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<ref_mut$<rhapsody::raw::disposal::SequentialDisposalItem> >; module=1978, inline_record=444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$19::eq
// 信息 ID: c2db9b9a3ac8e7b1f943f22e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; module=1978, inline_record=31524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$21::default
// 信息 ID: 8f1badbdb46ef1308a29edf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=1962, inline_record=5276; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert; module=1974, inline_record=2188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$4::dispose_to_bucket
// 信息 ID: 9a9496e8b332f81f1f005a6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=4636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$5::dispose_to_bucket
// 信息 ID: bf765b7d9bfa1d74789f3006
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=3624; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=684, inline_record=5164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$6::dispose_to_bucket
// 信息 ID: 750d5b8fe06d5fa22cb4433d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::buffer::impl$0::dispose; module=1921, inline_record=528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$7::dispose_to_bucket
// 信息 ID: 39d75234ed6404291f1b4182
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1987, inline_record=29316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$8::dispose_to_bucket
// 信息 ID: e014798cc7bfb245243a0565
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1987, inline_record=30056; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1987, inline_record=30808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$9::dispose_to_bucket
// 信息 ID: bc9520bab5f715d9d3356129
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1920, inline_record=7752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
