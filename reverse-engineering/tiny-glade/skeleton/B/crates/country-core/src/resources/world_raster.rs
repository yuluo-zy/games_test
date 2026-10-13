// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/world_raster.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b9c53f5fd9ecd76a51caf24a3915c30c8eff06c6344e437b5370bca11ecd38bd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 10, 173, 178, 247, 443, 467, 476, 502, 509, 524, 531, 536, 538, 539, 540, 556, 560, 563, 568, 569, 574, 596, 597, 1176, 1177, 1406, 1560, 1606, 1611, 1630, 1698, 1699, 1722, 2436, 2492, 2707, 2742, 3098, 3128, 3641, 3648, 3660, 3666, 3700, 3807, 3808, 3811, 3877, 3878, 3880, 3885, 3892, 4422, 4433, 4490, 4679
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.11"}]
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// use zeno; // namespace candidate
//   package identities: [{"name":"zeno","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f","version":"0.2.2"},{"name":"zeno","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::clear_buffer<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":14108,"rva":10574352,"va":5379283472}
// 编译实例 RVAs: 0xa15a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32385780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e8f42aee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379284183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32563102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e89edff001","size":5,"soft_fail":false,"target_va":5411847200,"va":5379284093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32257338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e83a35ec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379284241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e875efffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379283542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32259743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e89f3eec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379283612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2101494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e8f6102000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379284037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2106680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e838252000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379283955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2102328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e838142000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379284147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2101193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e8c90f2000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379283506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32252870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283472,"raw_hex":"e8c623ec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379284229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":16808,"parent_record_offset":14108,"ancestor_record_offset":14108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":16664,"parent_record_offset":14108,"ancestor_record_offset":14108,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":15696,"parent_record_offset":14108,"ancestor_record_offset":14108,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":14272,"parent_record_offset":14108,"ancestor_record_offset":14108,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":14720,"parent_record_offset":14108,"ancestor_record_offset":14108,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":14108,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":14108,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":16044,"parent_record_offset":16016,"ancestor_record_offset":14108,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":14108,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":15988,"parent_record_offset":15964,"ancestor_record_offset":14108,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":16160,"parent_record_offset":15988,"ancestor_record_offset":14108,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":14108,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":14108,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":14108,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":15832,"parent_record_offset":15792,"ancestor_record_offset":14108,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":14668,"parent_record_offset":14640,"ancestor_record_offset":14108,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":16620,"parent_record_offset":15792,"ancestor_record_offset":14108,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":16224,"parent_record_offset":15936,"ancestor_record_offset":14108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14108,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":14108,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14108,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":14108,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":14108,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":14108,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":15388,"parent_record_offset":15360,"ancestor_record_offset":14108,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":15036,"parent_record_offset":14920,"ancestor_record_offset":14108,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":16724,"parent_record_offset":16692,"ancestor_record_offset":14108,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":539,"record_offset":16692,"parent_record_offset":16664,"ancestor_record_offset":14108,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":14108,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":14108,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":14888,"parent_record_offset":14856,"ancestor_record_offset":14108,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":14856,"parent_record_offset":14820,"ancestor_record_offset":14108,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":14108,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":14820,"parent_record_offset":14796,"ancestor_record_offset":14108,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":14108,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":15360,"parent_record_offset":14820,"ancestor_record_offset":14108,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14360,"parent_record_offset":14328,"ancestor_record_offset":14108,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":14108,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14520,"parent_record_offset":14488,"ancestor_record_offset":14108,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14616,"parent_record_offset":14520,"ancestor_record_offset":14108,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14488,"parent_record_offset":14464,"ancestor_record_offset":14108,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14304,"parent_record_offset":14272,"ancestor_record_offset":14108,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":14108,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":14108,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":15204,"parent_record_offset":14920,"ancestor_record_offset":14108,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":15176,"parent_record_offset":14920,"ancestor_record_offset":14108,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":14980,"parent_record_offset":14920,"ancestor_record_offset":14108,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":15728,"parent_record_offset":15696,"ancestor_record_offset":14108,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":16872,"parent_record_offset":16840,"ancestor_record_offset":14108,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":17372,"parent_record_offset":17112,"ancestor_record_offset":14108,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":15860,"parent_record_offset":15832,"ancestor_record_offset":14108,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":15936,"parent_record_offset":15792,"ancestor_record_offset":14108,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":16196,"parent_record_offset":15936,"ancestor_record_offset":14108,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":15888,"parent_record_offset":15860,"ancestor_record_offset":14108,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":15792,"parent_record_offset":15760,"ancestor_record_offset":14108,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":15760,"parent_record_offset":15728,"ancestor_record_offset":14108,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":29556,"rva":10578272,"va":5379287392}
// 编译实例 RVAs: 0xa16960, 0x2261d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4206700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e894cfbfff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379288183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4207398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8daccbfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379288897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416bdab0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13265811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8936bca00","size":5,"soft_fail":false,"target_va":5392554672,"va":5379288856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416be2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13267964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8fc73ca00","size":5,"soft_fail":false,"target_va":5392556768,"va":5379288799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416beb50
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13270091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e84b7cca00","size":5,"soft_fail":false,"target_va":5392558928,"va":5379288832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32380904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8e817ee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379289059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t14167291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8fb2cd800","size":5,"soft_fail":false,"target_va":5393455584,"va":5379288288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32252752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e85023ec01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379288206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32252458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e82a22ec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379289121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32252612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8c422ec01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379289159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32252914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8f223ec01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379289145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-8240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8d0dfffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379287547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32255735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8f72eec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379287620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e832012000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379288073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2102648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e878152000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379287987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e860012000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379288971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e82b002000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379287504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7045032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404761360,"raw_hex":"e8a87f6b00","size":5,"soft_fail":false,"target_va":5411808064,"va":5404763027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9535875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e883819100","size":5,"soft_fail":false,"target_va":5388825296,"va":5379289416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32247990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8b610ec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379289109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::mask::Mask$LT$D$GT$::blend_into
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t3644012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404761360,"raw_hex":"e86c9a3700","size":5,"soft_fail":false,"target_va":5408406784,"va":5404762767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33032,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":32480,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":33852,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":32860,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":29924,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":539,"record_offset":35160,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":33884,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::ws_to_px
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":539,"record_offset":32948,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30978,"source_line":173,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":31496,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":30064,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":30520,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::format
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":33764,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30985,"source_line":120,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":33736,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30984,"source_line":69,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::size
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":33820,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30987,"source_line":140,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":33792,"parent_record_offset":29556,"ancestor_record_offset":29556,"inlinee":30986,"source_line":107,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32780,"parent_record_offset":32756,"ancestor_record_offset":29556,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32804,"parent_record_offset":32780,"ancestor_record_offset":29556,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":34072,"parent_record_offset":34048,"ancestor_record_offset":29556,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32756,"parent_record_offset":32580,"ancestor_record_offset":29556,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":34048,"parent_record_offset":33956,"ancestor_record_offset":29556,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":29556,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":29556,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33984,"parent_record_offset":33956,"ancestor_record_offset":29556,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33956,"parent_record_offset":33932,"ancestor_record_offset":29556,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33140,"parent_record_offset":33116,"ancestor_record_offset":29556,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":29556,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":32580,"parent_record_offset":32552,"ancestor_record_offset":29556,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":29556,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":29556,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":32616,"parent_record_offset":32580,"ancestor_record_offset":29556,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33068,"parent_record_offset":33032,"ancestor_record_offset":29556,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":32504,"parent_record_offset":32480,"ancestor_record_offset":29556,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":32640,"parent_record_offset":32616,"ancestor_record_offset":29556,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":32664,"parent_record_offset":32640,"ancestor_record_offset":29556,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31832,"parent_record_offset":31804,"ancestor_record_offset":29556,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":29556,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31860,"parent_record_offset":31832,"ancestor_record_offset":29556,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31780,"parent_record_offset":31752,"ancestor_record_offset":29556,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31804,"parent_record_offset":31780,"ancestor_record_offset":29556,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":31976,"parent_record_offset":31804,"ancestor_record_offset":29556,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":29556,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":29556,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":32688,"parent_record_offset":32664,"ancestor_record_offset":29556,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":29556,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":34016,"parent_record_offset":33984,"ancestor_record_offset":29556,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":30220,"parent_record_offset":30188,"ancestor_record_offset":29556,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":33644,"parent_record_offset":33620,"ancestor_record_offset":29556,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":29556,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":32996,"parent_record_offset":32972,"ancestor_record_offset":29556,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":31648,"parent_record_offset":31604,"ancestor_record_offset":29556,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":30468,"parent_record_offset":30440,"ancestor_record_offset":29556,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":32436,"parent_record_offset":31604,"ancestor_record_offset":29556,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":30020,"parent_record_offset":29996,"ancestor_record_offset":29556,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":32040,"parent_record_offset":31752,"ancestor_record_offset":29556,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":539,"record_offset":29996,"parent_record_offset":29972,"ancestor_record_offset":29556,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":30932,"parent_record_offset":30908,"ancestor_record_offset":29556,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":30908,"parent_record_offset":30884,"ancestor_record_offset":29556,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":29972,"parent_record_offset":29948,"ancestor_record_offset":29556,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":31116,"parent_record_offset":31088,"ancestor_record_offset":29556,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":29556,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":30860,"parent_record_offset":30836,"ancestor_record_offset":29556,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":30884,"parent_record_offset":30860,"ancestor_record_offset":29556,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":29948,"parent_record_offset":29924,"ancestor_record_offset":29556,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":31188,"parent_record_offset":31160,"ancestor_record_offset":29556,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":32884,"parent_record_offset":32860,"ancestor_record_offset":29556,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":30836,"parent_record_offset":30720,"ancestor_record_offset":29556,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":30188,"parent_record_offset":30160,"ancestor_record_offset":29556,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":30548,"parent_record_offset":30520,"ancestor_record_offset":29556,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":30688,"parent_record_offset":30656,"ancestor_record_offset":29556,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":30656,"parent_record_offset":30620,"ancestor_record_offset":29556,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":29556,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":30620,"parent_record_offset":30596,"ancestor_record_offset":29556,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":30572,"parent_record_offset":30548,"ancestor_record_offset":29556,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":31160,"parent_record_offset":30620,"ancestor_record_offset":29556,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30160,"parent_record_offset":30128,"ancestor_record_offset":29556,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30128,"parent_record_offset":30100,"ancestor_record_offset":29556,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30320,"parent_record_offset":30288,"ancestor_record_offset":29556,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30416,"parent_record_offset":30320,"ancestor_record_offset":29556,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":29556,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30100,"parent_record_offset":30064,"ancestor_record_offset":29556,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":30440,"parent_record_offset":30416,"ancestor_record_offset":29556,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":30720,"parent_record_offset":30688,"ancestor_record_offset":29556,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":31004,"parent_record_offset":30720,"ancestor_record_offset":29556,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":30976,"parent_record_offset":30720,"ancestor_record_offset":29556,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":30780,"parent_record_offset":30720,"ancestor_record_offset":29556,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":33676,"parent_record_offset":33532,"ancestor_record_offset":29556,"inlinee":29552,"source_line":1182,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":33620,"parent_record_offset":33532,"ancestor_record_offset":29556,"inlinee":30982,"source_line":1367,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":32972,"parent_record_offset":32948,"ancestor_record_offset":29556,"inlinee":30980,"source_line":1047,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":31532,"parent_record_offset":31496,"ancestor_record_offset":29556,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":34184,"parent_record_offset":34152,"ancestor_record_offset":29556,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":34684,"parent_record_offset":34424,"ancestor_record_offset":29556,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":31676,"parent_record_offset":31648,"ancestor_record_offset":29556,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":31752,"parent_record_offset":31604,"ancestor_record_offset":29556,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":32012,"parent_record_offset":31752,"ancestor_record_offset":29556,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":31704,"parent_record_offset":31676,"ancestor_record_offset":29556,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":31604,"parent_record_offset":31568,"ancestor_record_offset":29556,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":31568,"parent_record_offset":31532,"ancestor_record_offset":29556,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":35540,"rva":10578272,"va":5379287392}
// 编译实例 RVAs: 0xa16960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4206700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e894cfbfff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379288183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4207398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8daccbfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379288897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416bdab0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13265811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8936bca00","size":5,"soft_fail":false,"target_va":5392554672,"va":5379288856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416be2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13267964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8fc73ca00","size":5,"soft_fail":false,"target_va":5392556768,"va":5379288799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416beb50
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13270091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e84b7cca00","size":5,"soft_fail":false,"target_va":5392558928,"va":5379288832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32380904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8e817ee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379289059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14167291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8fb2cd800","size":5,"soft_fail":false,"target_va":5393455584,"va":5379288288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32252752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e85023ec01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379288206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32252458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e82a22ec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379289121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32252612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8c422ec01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379289159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32252914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8f223ec01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379289145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8d0dfffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379287547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32255735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8f72eec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379287620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e832012000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379288073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2102648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e878152000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379287987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e860012000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379288971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2097195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e82b002000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379287504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9535875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e883819100","size":5,"soft_fail":false,"target_va":5388825296,"va":5379289416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32247990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379287392,"raw_hex":"e8b610ec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379289109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":39016,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":38464,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":39836,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":38844,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":35908,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":539,"record_offset":41144,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":39868,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::ws_to_px
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":539,"record_offset":38932,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30978,"source_line":173,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":37480,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":36048,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":36504,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::format
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":39748,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30985,"source_line":120,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":39720,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30984,"source_line":69,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":39804,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30987,"source_line":140,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":539,"record_offset":39776,"parent_record_offset":35540,"ancestor_record_offset":35540,"inlinee":30986,"source_line":107,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":38764,"parent_record_offset":38740,"ancestor_record_offset":35540,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":38788,"parent_record_offset":38764,"ancestor_record_offset":35540,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":35540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":38740,"parent_record_offset":38564,"ancestor_record_offset":35540,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":40032,"parent_record_offset":39940,"ancestor_record_offset":35540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39076,"parent_record_offset":39052,"ancestor_record_offset":35540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":38512,"parent_record_offset":38488,"ancestor_record_offset":35540,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39968,"parent_record_offset":39940,"ancestor_record_offset":35540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39940,"parent_record_offset":39916,"ancestor_record_offset":35540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":35540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":35540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":38564,"parent_record_offset":38536,"ancestor_record_offset":35540,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":38536,"parent_record_offset":38512,"ancestor_record_offset":35540,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":39916,"parent_record_offset":39892,"ancestor_record_offset":35540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":38600,"parent_record_offset":38564,"ancestor_record_offset":35540,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":39052,"parent_record_offset":39016,"ancestor_record_offset":35540,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":38488,"parent_record_offset":38464,"ancestor_record_offset":35540,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":38624,"parent_record_offset":38600,"ancestor_record_offset":35540,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":38648,"parent_record_offset":38624,"ancestor_record_offset":35540,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37816,"parent_record_offset":37788,"ancestor_record_offset":35540,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37868,"parent_record_offset":37844,"ancestor_record_offset":35540,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37844,"parent_record_offset":37816,"ancestor_record_offset":35540,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37764,"parent_record_offset":37736,"ancestor_record_offset":35540,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37788,"parent_record_offset":37764,"ancestor_record_offset":35540,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":37960,"parent_record_offset":37788,"ancestor_record_offset":35540,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":38072,"parent_record_offset":38048,"ancestor_record_offset":35540,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":37892,"parent_record_offset":37868,"ancestor_record_offset":35540,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":35540,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":35540,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":40000,"parent_record_offset":39968,"ancestor_record_offset":35540,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":36204,"parent_record_offset":36172,"ancestor_record_offset":35540,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":39628,"parent_record_offset":39604,"ancestor_record_offset":35540,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":39684,"parent_record_offset":39660,"ancestor_record_offset":35540,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":38980,"parent_record_offset":38956,"ancestor_record_offset":35540,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":37632,"parent_record_offset":37588,"ancestor_record_offset":35540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":36452,"parent_record_offset":36424,"ancestor_record_offset":35540,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":38420,"parent_record_offset":37588,"ancestor_record_offset":35540,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":36004,"parent_record_offset":35980,"ancestor_record_offset":35540,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":38024,"parent_record_offset":37736,"ancestor_record_offset":35540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":539,"record_offset":35980,"parent_record_offset":35956,"ancestor_record_offset":35540,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":36916,"parent_record_offset":36892,"ancestor_record_offset":35540,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":36892,"parent_record_offset":36868,"ancestor_record_offset":35540,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":35956,"parent_record_offset":35932,"ancestor_record_offset":35540,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":37100,"parent_record_offset":37072,"ancestor_record_offset":35540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":37916,"parent_record_offset":37892,"ancestor_record_offset":35540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":36844,"parent_record_offset":36820,"ancestor_record_offset":35540,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":36868,"parent_record_offset":36844,"ancestor_record_offset":35540,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":35932,"parent_record_offset":35908,"ancestor_record_offset":35540,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":35540,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":38868,"parent_record_offset":38844,"ancestor_record_offset":35540,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":36820,"parent_record_offset":36704,"ancestor_record_offset":35540,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":36172,"parent_record_offset":36144,"ancestor_record_offset":35540,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":36532,"parent_record_offset":36504,"ancestor_record_offset":35540,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":36672,"parent_record_offset":36640,"ancestor_record_offset":35540,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":36640,"parent_record_offset":36604,"ancestor_record_offset":35540,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":36580,"parent_record_offset":36556,"ancestor_record_offset":35540,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":36604,"parent_record_offset":36580,"ancestor_record_offset":35540,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":36556,"parent_record_offset":36532,"ancestor_record_offset":35540,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":37144,"parent_record_offset":36604,"ancestor_record_offset":35540,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36144,"parent_record_offset":36112,"ancestor_record_offset":35540,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36112,"parent_record_offset":36084,"ancestor_record_offset":35540,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36304,"parent_record_offset":36272,"ancestor_record_offset":35540,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36400,"parent_record_offset":36304,"ancestor_record_offset":35540,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":35540,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36084,"parent_record_offset":36048,"ancestor_record_offset":35540,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":36424,"parent_record_offset":36400,"ancestor_record_offset":35540,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":36704,"parent_record_offset":36672,"ancestor_record_offset":35540,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":36988,"parent_record_offset":36704,"ancestor_record_offset":35540,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":36960,"parent_record_offset":36704,"ancestor_record_offset":35540,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":36764,"parent_record_offset":36704,"ancestor_record_offset":35540,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":39660,"parent_record_offset":39516,"ancestor_record_offset":35540,"inlinee":29552,"source_line":1182,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":39604,"parent_record_offset":39516,"ancestor_record_offset":35540,"inlinee":30982,"source_line":1367,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":38956,"parent_record_offset":38932,"ancestor_record_offset":35540,"inlinee":30980,"source_line":1047,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":37516,"parent_record_offset":37480,"ancestor_record_offset":35540,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":40168,"parent_record_offset":40136,"ancestor_record_offset":35540,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":40668,"parent_record_offset":40408,"ancestor_record_offset":35540,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":37660,"parent_record_offset":37632,"ancestor_record_offset":35540,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":37736,"parent_record_offset":37588,"ancestor_record_offset":35540,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":37996,"parent_record_offset":37736,"ancestor_record_offset":35540,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":37688,"parent_record_offset":37660,"ancestor_record_offset":35540,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":37588,"parent_record_offset":37552,"ancestor_record_offset":35540,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":37552,"parent_record_offset":37516,"ancestor_record_offset":35540,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::unique
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7175
// 首条位置: {"module":509,"record_offset":71784,"rva":9671600,"va":5378380720}
// 编译实例 RVAs: 0x9393b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":509,"record_offset":71936,"parent_record_offset":71784,"ancestor_record_offset":71784,"inlinee":18371,"source_line":50,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::shared
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7176
// 首条位置: {"module":596,"inlinee":29046}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":596,"record_offset":108116,"parent_record_offset":108056,"ancestor_record_offset":108056,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":596,"record_offset":108088,"parent_record_offset":108056,"ancestor_record_offset":108056,"inlinee":18371,"source_line":50,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":108240,"parent_record_offset":108216,"ancestor_record_offset":108056,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":108264,"parent_record_offset":108240,"ancestor_record_offset":108056,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":108184,"parent_record_offset":108148,"ancestor_record_offset":108056,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":108216,"parent_record_offset":108184,"ancestor_record_offset":108056,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":596,"record_offset":108148,"parent_record_offset":108116,"ancestor_record_offset":108056,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":108288,"parent_record_offset":108264,"ancestor_record_offset":108056,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::secondary
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7177
// 首条位置: {"module":596,"inlinee":29047}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":596,"record_offset":108476,"parent_record_offset":108440,"ancestor_record_offset":108440,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":596,"record_offset":108564,"parent_record_offset":108440,"ancestor_record_offset":108440,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":596,"record_offset":108504,"parent_record_offset":108476,"ancestor_record_offset":108440,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":596,"record_offset":108528,"parent_record_offset":108504,"ancestor_record_offset":108440,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::new<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":11744,"rva":10573104,"va":5379282224}
// 编译实例 RVAs: 0xa15530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4200909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e833e6bfff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379282392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4201408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e840e4bfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379282907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4200758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e8cae6bfff","size":5,"soft_fail":false,"target_va":5375081536,"va":5379282289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e8fbf0ffff","size":5,"soft_fail":false,"target_va":5379279136,"va":5379282976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32564597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e875e5f001","size":5,"soft_fail":false,"target_va":5411847200,"va":5379282598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32258229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379282224,"raw_hex":"e8b538ec01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379282729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":11892,"parent_record_offset":11744,"ancestor_record_offset":11744,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":12980,"parent_record_offset":11744,"ancestor_record_offset":11744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":11744,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":11744,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":11744,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":11744,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":12568,"parent_record_offset":12540,"ancestor_record_offset":11744,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11744,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":11744,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":11744,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":11948,"parent_record_offset":11920,"ancestor_record_offset":11744,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":13052,"parent_record_offset":13028,"ancestor_record_offset":11744,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":12008,"parent_record_offset":11976,"ancestor_record_offset":11744,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":11744,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":11976,"parent_record_offset":11948,"ancestor_record_offset":11744,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":13028,"parent_record_offset":13004,"ancestor_record_offset":11744,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":12668,"parent_record_offset":12416,"ancestor_record_offset":11744,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":12452,"parent_record_offset":12416,"ancestor_record_offset":11744,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":539,"record_offset":12416,"parent_record_offset":12384,"ancestor_record_offset":11744,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":539,"record_offset":11920,"parent_record_offset":11892,"ancestor_record_offset":11744,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":11744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":11744,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::as_float_grid
// 记录形式: inline_instance, inlinee_declaration; instances=71
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6344
// 首条位置: {"module":443,"inlinee":25412}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::ws_to_px
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7902
// 首条位置: {"module":539,"inlinee":30978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":33676,"parent_record_offset":33532,"ancestor_record_offset":33532,"inlinee":29552,"source_line":1182,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":33620,"parent_record_offset":33532,"ancestor_record_offset":33532,"inlinee":30982,"source_line":1367,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":539,"record_offset":32972,"parent_record_offset":32948,"ancestor_record_offset":32948,"inlinee":30980,"source_line":1047,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":33644,"parent_record_offset":33620,"ancestor_record_offset":33532,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":33532,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":539,"record_offset":32996,"parent_record_offset":32972,"ancestor_record_offset":32948,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::subscribe<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":13488,"rva":10574048,"va":5379283168}
// 编译实例 RVAs: 0xa158e0, 0x2612be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9125321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e837c274ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5379283444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141454290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10742082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e842e9a300","size":5,"soft_fail":false,"target_va":5390025360,"va":5379283273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf68e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9552750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408631776,"raw_hex":"e8923c6eff","size":5,"soft_fail":false,"target_va":5399079136,"va":5408631881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32258496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e8c039ec01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379283403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32209024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e88078eb01","size":5,"soft_fail":false,"target_va":5411492368,"va":5379283339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32211444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e8f481eb01","size":5,"soft_fail":false,"target_va":5411494800,"va":5379283351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2540334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e82ec32600","size":5,"soft_fail":false,"target_va":5381823600,"va":5379283261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2540723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379283168,"raw_hex":"e8b3c42600","size":5,"soft_fail":false,"target_va":5381824016,"va":5379283288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141454290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10742030","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379283168,"raw_hex":"e90ee9a300","size":5,"soft_fail":false,"target_va":5390025360,"va":5379283325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141cf68e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-9552802","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5408631776,"raw_hex":"e95e3c6eff","size":5,"soft_fail":false,"target_va":5399079136,"va":5408631933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":539,"record_offset":13656,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":539,"record_offset":14076,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":13896,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":13684,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":13840,"parent_record_offset":13488,"ancestor_record_offset":13488,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13488,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":14008,"parent_record_offset":13948,"ancestor_record_offset":13488,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":13748,"parent_record_offset":13712,"ancestor_record_offset":13488,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":13488,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":13488,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13488,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":539,"record_offset":13712,"parent_record_offset":13684,"ancestor_record_offset":13488,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":539,"record_offset":13948,"parent_record_offset":13920,"ancestor_record_offset":13488,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":539,"record_offset":13804,"parent_record_offset":13712,"ancestor_record_offset":13488,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":539,"record_offset":13980,"parent_record_offset":13948,"ancestor_record_offset":13488,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::last_staged
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x664F
// 首条位置: {"module":476,"inlinee":26191}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::replace_composite
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6651
// 首条位置: {"module":476,"inlinee":26193}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":476,"record_offset":70680,"parent_record_offset":70584,"ancestor_record_offset":70584,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":70624,"parent_record_offset":70584,"ancestor_record_offset":70584,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":70648,"parent_record_offset":70624,"ancestor_record_offset":70584,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::clear_composite
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6650
// 首条位置: {"module":476,"inlinee":26192}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70256,"parent_record_offset":70232,"ancestor_record_offset":70232,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":70464,"parent_record_offset":70232,"ancestor_record_offset":70232,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70364,"parent_record_offset":70340,"ancestor_record_offset":70232,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70412,"parent_record_offset":70388,"ancestor_record_offset":70232,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70388,"parent_record_offset":70364,"ancestor_record_offset":70232,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70340,"parent_record_offset":70312,"ancestor_record_offset":70232,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70312,"parent_record_offset":70284,"ancestor_record_offset":70232,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70284,"parent_record_offset":70256,"ancestor_record_offset":70232,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":70492,"parent_record_offset":70464,"ancestor_record_offset":70232,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::prepare_tile
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7915
// 首条位置: {"module":568,"inlinee":30997}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":70588,"parent_record_offset":69732,"ancestor_record_offset":69732,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70032,"parent_record_offset":69732,"ancestor_record_offset":69732,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70924,"parent_record_offset":69732,"ancestor_record_offset":69732,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":568,"record_offset":70752,"parent_record_offset":70588,"ancestor_record_offset":69732,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70376,"parent_record_offset":70172,"ancestor_record_offset":69732,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70948,"parent_record_offset":70924,"ancestor_record_offset":69732,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70096,"parent_record_offset":70032,"ancestor_record_offset":69732,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x792F
// 首条位置: {"module":1630,"inlinee":31023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":494696,"parent_record_offset":494668,"ancestor_record_offset":494668,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":494720,"parent_record_offset":494696,"ancestor_record_offset":494668,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":494744,"parent_record_offset":494720,"ancestor_record_offset":494668,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":494768,"parent_record_offset":494744,"ancestor_record_offset":494668,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1226A
// 首条位置: {"module":1630,"inlinee":74346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":494576,"parent_record_offset":494548,"ancestor_record_offset":494548,"inlinee":74331,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":495132,"parent_record_offset":495108,"ancestor_record_offset":494548,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":495108,"parent_record_offset":495040,"ancestor_record_offset":494548,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":495076,"parent_record_offset":495040,"ancestor_record_offset":494548,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":495040,"parent_record_offset":495012,"ancestor_record_offset":494548,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":495012,"parent_record_offset":494984,"ancestor_record_offset":494548,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":494720,"parent_record_offset":494696,"ancestor_record_offset":494548,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":494928,"parent_record_offset":494632,"ancestor_record_offset":494548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":494744,"parent_record_offset":494720,"ancestor_record_offset":494548,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":494768,"parent_record_offset":494744,"ancestor_record_offset":494548,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":494696,"parent_record_offset":494668,"ancestor_record_offset":494548,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1630,"record_offset":494668,"parent_record_offset":494632,"ancestor_record_offset":494548,"inlinee":31023,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":494632,"parent_record_offset":494604,"ancestor_record_offset":494548,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1630,"record_offset":494604,"parent_record_offset":494576,"ancestor_record_offset":494548,"inlinee":74332,"source_line":615,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7911
// 首条位置: {"module":539,"inlinee":30993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":539,"record_offset":71396,"parent_record_offset":71336,"ancestor_record_offset":71336,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":539,"record_offset":71428,"parent_record_offset":71336,"ancestor_record_offset":71336,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":539,"record_offset":71368,"parent_record_offset":71336,"ancestor_record_offset":71336,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7930
// 首条位置: {"module":1630,"inlinee":31024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":654164,"parent_record_offset":654140,"ancestor_record_offset":654140,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":654188,"parent_record_offset":654164,"ancestor_record_offset":654140,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654212,"parent_record_offset":654188,"ancestor_record_offset":654140,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654236,"parent_record_offset":654212,"ancestor_record_offset":654140,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122EC
// 首条位置: {"module":1630,"inlinee":74476}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":654040,"parent_record_offset":654016,"ancestor_record_offset":654016,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":654460,"parent_record_offset":654436,"ancestor_record_offset":654016,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":654436,"parent_record_offset":654380,"ancestor_record_offset":654016,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":654408,"parent_record_offset":654380,"ancestor_record_offset":654016,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":654380,"parent_record_offset":654356,"ancestor_record_offset":654016,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":654356,"parent_record_offset":654332,"ancestor_record_offset":654016,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":654188,"parent_record_offset":654164,"ancestor_record_offset":654016,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":654284,"parent_record_offset":654112,"ancestor_record_offset":654016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654212,"parent_record_offset":654188,"ancestor_record_offset":654016,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654236,"parent_record_offset":654212,"ancestor_record_offset":654016,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":654164,"parent_record_offset":654140,"ancestor_record_offset":654016,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1630,"record_offset":654140,"parent_record_offset":654112,"ancestor_record_offset":654016,"inlinee":31024,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":654112,"parent_record_offset":654088,"ancestor_record_offset":654016,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":654064,"parent_record_offset":654040,"ancestor_record_offset":654016,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":654088,"parent_record_offset":654064,"ancestor_record_offset":654016,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x122EA
// 首条位置: {"module":1630,"inlinee":74474}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":655328,"parent_record_offset":652488,"ancestor_record_offset":652488,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":655364,"parent_record_offset":652488,"ancestor_record_offset":652488,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":652528,"parent_record_offset":652488,"ancestor_record_offset":652488,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":654776,"parent_record_offset":652488,"ancestor_record_offset":652488,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":653328,"parent_record_offset":653304,"ancestor_record_offset":652488,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":653304,"parent_record_offset":653248,"ancestor_record_offset":652488,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":652672,"parent_record_offset":652648,"ancestor_record_offset":652488,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":652724,"parent_record_offset":652696,"ancestor_record_offset":652488,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":652748,"parent_record_offset":652724,"ancestor_record_offset":652488,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653684,"parent_record_offset":653660,"ancestor_record_offset":652488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653276,"parent_record_offset":653248,"ancestor_record_offset":652488,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653248,"parent_record_offset":653224,"ancestor_record_offset":652488,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653732,"parent_record_offset":653708,"ancestor_record_offset":652488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653708,"parent_record_offset":653684,"ancestor_record_offset":652488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":653224,"parent_record_offset":653200,"ancestor_record_offset":652488,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":653660,"parent_record_offset":653636,"ancestor_record_offset":652488,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":653636,"parent_record_offset":653380,"ancestor_record_offset":652488,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":654188,"parent_record_offset":654164,"ancestor_record_offset":652488,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":654892,"parent_record_offset":654868,"ancestor_record_offset":652488,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":653592,"parent_record_offset":653564,"ancestor_record_offset":652488,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":652696,"parent_record_offset":652672,"ancestor_record_offset":652488,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":654868,"parent_record_offset":654832,"ancestor_record_offset":652488,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":652864,"parent_record_offset":652776,"ancestor_record_offset":652488,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":653092,"parent_record_offset":652600,"ancestor_record_offset":652488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":652888,"parent_record_offset":652864,"ancestor_record_offset":652488,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":654948,"parent_record_offset":654892,"ancestor_record_offset":652488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654212,"parent_record_offset":654188,"ancestor_record_offset":652488,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":654236,"parent_record_offset":654212,"ancestor_record_offset":652488,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":652776,"parent_record_offset":652748,"ancestor_record_offset":652488,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":652832,"parent_record_offset":652808,"ancestor_record_offset":652488,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":652808,"parent_record_offset":652776,"ancestor_record_offset":652488,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":654164,"parent_record_offset":654140,"ancestor_record_offset":652488,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1630,"record_offset":654140,"parent_record_offset":654112,"ancestor_record_offset":652488,"inlinee":31024,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1630,"record_offset":654016,"parent_record_offset":653992,"ancestor_record_offset":652488,"inlinee":74476,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":655436,"parent_record_offset":655412,"ancestor_record_offset":652488,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":655412,"parent_record_offset":655388,"ancestor_record_offset":652488,"inlinee":6671,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":652564,"parent_record_offset":652528,"ancestor_record_offset":652488,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":654804,"parent_record_offset":654776,"ancestor_record_offset":652488,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":654112,"parent_record_offset":654088,"ancestor_record_offset":652488,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":655008,"parent_record_offset":654984,"ancestor_record_offset":652488,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":653992,"parent_record_offset":652600,"ancestor_record_offset":652488,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":655284,"parent_record_offset":655260,"ancestor_record_offset":652488,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":652648,"parent_record_offset":652600,"ancestor_record_offset":652488,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":654064,"parent_record_offset":654040,"ancestor_record_offset":652488,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":654040,"parent_record_offset":654016,"ancestor_record_offset":652488,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":654088,"parent_record_offset":654064,"ancestor_record_offset":652488,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":652600,"parent_record_offset":652564,"ancestor_record_offset":652488,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":654832,"parent_record_offset":654804,"ancestor_record_offset":652488,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":70500,"rva":10590144,"va":5379299264}
// 编译实例 RVAs: 0xa197c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32178214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e82600eb01","size":5,"soft_fail":false,"target_va":5411478016,"va":5379299797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1500413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8fde41600","size":5,"soft_fail":false,"target_va":5380800512,"va":5379300094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4218705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8afa0bfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379300204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1819379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e80d3de4ff","size":5,"soft_fail":false,"target_va":5377481360,"va":5379300734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140887df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1645304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e808e5e6ff","size":5,"soft_fail":false,"target_va":5377654256,"va":5379299555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1641636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e85cf3e6ff","size":5,"soft_fail":false,"target_va":5377658224,"va":5379299855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889390
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1640512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8c0f7e6ff","size":5,"soft_fail":false,"target_va":5377659792,"va":5379300299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1576003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8bdf3e7ff","size":5,"soft_fail":false,"target_va":5377724224,"va":5379300222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10628083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8f32ba200","size":5,"soft_fail":false,"target_va":5389927504,"va":5379299416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415d35c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12295303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e8879cbb00","size":5,"soft_fail":false,"target_va":5391594944,"va":5379299636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e5fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12371469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e80dc6bc00","size":5,"soft_fail":false,"target_va":5391671200,"va":5379299726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f9b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32375410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e87202ee01","size":5,"soft_fail":false,"target_va":5411674944,"va":5379299529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32178765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379299264,"raw_hex":"e84d02eb01","size":5,"soft_fail":false,"target_va":5411478544,"va":5379299774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":539,"record_offset":70680,"parent_record_offset":70500,"ancestor_record_offset":70500,"inlinee":26604,"source_line":255,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":71824,"parent_record_offset":71800,"ancestor_record_offset":70500,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":71800,"parent_record_offset":71712,"ancestor_record_offset":70500,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":539,"record_offset":70860,"parent_record_offset":70836,"ancestor_record_offset":70500,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":539,"record_offset":70916,"parent_record_offset":70892,"ancestor_record_offset":70500,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":539,"record_offset":70968,"parent_record_offset":70940,"ancestor_record_offset":70500,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":539,"record_offset":70992,"parent_record_offset":70968,"ancestor_record_offset":70500,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71572,"parent_record_offset":71548,"ancestor_record_offset":70500,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71740,"parent_record_offset":71712,"ancestor_record_offset":70500,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71712,"parent_record_offset":71688,"ancestor_record_offset":70500,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71620,"parent_record_offset":71596,"ancestor_record_offset":70500,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71596,"parent_record_offset":71572,"ancestor_record_offset":70500,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":71688,"parent_record_offset":71664,"ancestor_record_offset":70500,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":71548,"parent_record_offset":71524,"ancestor_record_offset":70500,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":71524,"parent_record_offset":71496,"ancestor_record_offset":70500,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":71768,"parent_record_offset":71740,"ancestor_record_offset":70500,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":539,"record_offset":70940,"parent_record_offset":70916,"ancestor_record_offset":70500,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":71108,"parent_record_offset":71020,"ancestor_record_offset":70500,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":71460,"parent_record_offset":70680,"ancestor_record_offset":70500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":71132,"parent_record_offset":71108,"ancestor_record_offset":70500,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":539,"record_offset":71396,"parent_record_offset":71336,"ancestor_record_offset":70500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":539,"record_offset":71428,"parent_record_offset":71336,"ancestor_record_offset":70500,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":71020,"parent_record_offset":70992,"ancestor_record_offset":70500,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":71076,"parent_record_offset":71052,"ancestor_record_offset":70500,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":71052,"parent_record_offset":71020,"ancestor_record_offset":70500,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":539,"record_offset":71336,"parent_record_offset":70680,"ancestor_record_offset":70500,"inlinee":30993,"source_line":261,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":539,"record_offset":71368,"parent_record_offset":71336,"ancestor_record_offset":70500,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":539,"record_offset":70836,"parent_record_offset":70680,"ancestor_record_offset":70500,"inlinee":26606,"source_line":877,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":539,"record_offset":70892,"parent_record_offset":70680,"ancestor_record_offset":70500,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":240,"rva":246704,"va":5368955824}
// 编译实例 RVAs: 0x3c3b0, 0xa195a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4693248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368955824,"raw_hex":"e8009d4700","size":5,"soft_fail":false,"target_va":5373649152,"va":5368955899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4724753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368955824,"raw_hex":"e811184800","size":5,"soft_fail":false,"target_va":5373680768,"va":5368956010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141580d20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11958064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379298720,"raw_hex":"e83077b600","size":5,"soft_fail":false,"target_va":5391256864,"va":5379298795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11979313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379298720,"raw_hex":"e831cab600","size":5,"soft_fail":false,"target_va":5391278224,"va":5379298906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t42362075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368955824,"raw_hex":"e8db648602","size":5,"soft_fail":false,"target_va":5411318240,"va":5368956160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42521485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368955824,"raw_hex":"e88dd38802","size":5,"soft_fail":false,"target_va":5411477664,"va":5368956174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t42521466","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5368955824,"raw_hex":"e97ad38802","size":5,"soft_fail":false,"target_va":5411477664,"va":5368956193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":2208,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5726,"source_line":808,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":2152,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":524,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":3216,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":5432,"source_line":249,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":240,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":240,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":240,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":240,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":240,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":240,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":240,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":888,"parent_record_offset":720,"ancestor_record_offset":240,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":240,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":1008,"parent_record_offset":720,"ancestor_record_offset":240,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":760,"parent_record_offset":720,"ancestor_record_offset":240,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":10,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":240,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":1120,"parent_record_offset":1088,"ancestor_record_offset":240,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":10,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":240,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":580,"parent_record_offset":524,"ancestor_record_offset":240,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":1156,"parent_record_offset":524,"ancestor_record_offset":240,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":240,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":1636,"parent_record_offset":524,"ancestor_record_offset":240,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":1060,"parent_record_offset":524,"ancestor_record_offset":240,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":240,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":240,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":240,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$14::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F2
// 首条位置: {"module":539,"inlinee":30962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::clear_all<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":17852,"rva":10575264,"va":5379284384}
// 编译实例 RVAs: 0xa15da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32384729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8d926ee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379285234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32562216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e828dcf001","size":5,"soft_fail":false,"target_va":5411847200,"va":5379284979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32256287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e81f31ec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379285292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8eaebffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379284449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32258836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8143bec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379284519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2100582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8660d2000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379284949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2105768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8a8212000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379284867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2101275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e81b102000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379285200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2100285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e83d0c2000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379284414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32251819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379284384,"raw_hex":"e8ab1fec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379285280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":20904,"parent_record_offset":17852,"ancestor_record_offset":17852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":20408,"parent_record_offset":17852,"ancestor_record_offset":17852,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":19440,"parent_record_offset":17852,"ancestor_record_offset":17852,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":18016,"parent_record_offset":17852,"ancestor_record_offset":17852,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":18464,"parent_record_offset":17852,"ancestor_record_offset":17852,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19760,"parent_record_offset":19732,"ancestor_record_offset":17852,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19812,"parent_record_offset":19788,"ancestor_record_offset":17852,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19788,"parent_record_offset":19760,"ancestor_record_offset":17852,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":17852,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":17852,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":19904,"parent_record_offset":19732,"ancestor_record_offset":17852,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":17852,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":17852,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":18164,"parent_record_offset":18132,"ancestor_record_offset":17852,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":19576,"parent_record_offset":19536,"ancestor_record_offset":17852,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":17852,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":20364,"parent_record_offset":19536,"ancestor_record_offset":17852,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":19968,"parent_record_offset":19680,"ancestor_record_offset":17852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":20856,"parent_record_offset":20832,"ancestor_record_offset":17852,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":17852,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":18852,"parent_record_offset":18828,"ancestor_record_offset":17852,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":19060,"parent_record_offset":19032,"ancestor_record_offset":17852,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":19860,"parent_record_offset":19836,"ancestor_record_offset":17852,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":17852,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":17852,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":20832,"parent_record_offset":20808,"ancestor_record_offset":17852,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":19132,"parent_record_offset":19104,"ancestor_record_offset":17852,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":18780,"parent_record_offset":18664,"ancestor_record_offset":17852,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":17852,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":20808,"parent_record_offset":20724,"ancestor_record_offset":17852,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":539,"record_offset":20436,"parent_record_offset":20408,"ancestor_record_offset":17852,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":18132,"parent_record_offset":18104,"ancestor_record_offset":17852,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":18492,"parent_record_offset":18464,"ancestor_record_offset":17852,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":18632,"parent_record_offset":18600,"ancestor_record_offset":17852,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":18600,"parent_record_offset":18564,"ancestor_record_offset":17852,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":17852,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":17852,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":17852,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":19104,"parent_record_offset":18564,"ancestor_record_offset":17852,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18104,"parent_record_offset":18072,"ancestor_record_offset":17852,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18072,"parent_record_offset":18048,"ancestor_record_offset":17852,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18264,"parent_record_offset":18232,"ancestor_record_offset":17852,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18360,"parent_record_offset":18264,"ancestor_record_offset":17852,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":17852,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18048,"parent_record_offset":18016,"ancestor_record_offset":17852,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":17852,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":18664,"parent_record_offset":18632,"ancestor_record_offset":17852,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":18948,"parent_record_offset":18664,"ancestor_record_offset":17852,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":18920,"parent_record_offset":18664,"ancestor_record_offset":17852,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":18724,"parent_record_offset":18664,"ancestor_record_offset":17852,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":19472,"parent_record_offset":19440,"ancestor_record_offset":17852,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":20968,"parent_record_offset":20936,"ancestor_record_offset":17852,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":21468,"parent_record_offset":21208,"ancestor_record_offset":17852,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":17852,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":19680,"parent_record_offset":19536,"ancestor_record_offset":17852,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":19940,"parent_record_offset":19680,"ancestor_record_offset":17852,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":19632,"parent_record_offset":19604,"ancestor_record_offset":17852,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":19536,"parent_record_offset":19504,"ancestor_record_offset":17852,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":19504,"parent_record_offset":19472,"ancestor_record_offset":17852,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::copy_composite_to_scratch<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":21948,"rva":10576288,"va":5379285408}
// 编译实例 RVAs: 0xa161a0, 0x2262680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32383832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e85823ee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379286131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32561121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e8e1d7f001","size":5,"soft_fail":false,"target_va":5411847168,"va":5379286042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32255366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e8862dec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379286213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32257060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e82434ec01","size":5,"soft_fail":false,"target_va":5411543264,"va":5379286199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e8ede7ffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379285470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32257812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e81437ec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379285543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2099542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e856092000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379285989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2104728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e8981d2000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379285907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2100378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e89a0c2000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379286097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2099261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e83d082000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379285438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7043560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404763776,"raw_hex":"e8e8796b00","size":5,"soft_fail":false,"target_va":5411808064,"va":5404764499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9538963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e8938d9100","size":5,"soft_fail":false,"target_va":5388825296,"va":5379286328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32250922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379285408,"raw_hex":"e82a1cec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379286177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":24596,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":24528,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":23560,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":22128,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":22584,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":23880,"parent_record_offset":23852,"ancestor_record_offset":21948,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":23932,"parent_record_offset":23908,"ancestor_record_offset":21948,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":23908,"parent_record_offset":23880,"ancestor_record_offset":21948,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":23828,"parent_record_offset":23800,"ancestor_record_offset":21948,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":23852,"parent_record_offset":23828,"ancestor_record_offset":21948,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":24024,"parent_record_offset":23852,"ancestor_record_offset":21948,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":21948,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":24564,"parent_record_offset":24528,"ancestor_record_offset":21948,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":21948,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":22284,"parent_record_offset":22252,"ancestor_record_offset":21948,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":23696,"parent_record_offset":23656,"ancestor_record_offset":21948,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":22532,"parent_record_offset":22504,"ancestor_record_offset":21948,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":24484,"parent_record_offset":23656,"ancestor_record_offset":21948,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":24088,"parent_record_offset":23800,"ancestor_record_offset":21948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":21948,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":22972,"parent_record_offset":22948,"ancestor_record_offset":21948,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":23180,"parent_record_offset":23152,"ancestor_record_offset":21948,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":21948,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":22924,"parent_record_offset":22900,"ancestor_record_offset":21948,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":22948,"parent_record_offset":22924,"ancestor_record_offset":21948,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":23252,"parent_record_offset":23224,"ancestor_record_offset":21948,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":22900,"parent_record_offset":22784,"ancestor_record_offset":21948,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":22252,"parent_record_offset":22224,"ancestor_record_offset":21948,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":22612,"parent_record_offset":22584,"ancestor_record_offset":21948,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":22752,"parent_record_offset":22720,"ancestor_record_offset":21948,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":22720,"parent_record_offset":22684,"ancestor_record_offset":21948,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":22660,"parent_record_offset":22636,"ancestor_record_offset":21948,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":21948,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":21948,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":23224,"parent_record_offset":22684,"ancestor_record_offset":21948,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22224,"parent_record_offset":22192,"ancestor_record_offset":21948,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22192,"parent_record_offset":22164,"ancestor_record_offset":21948,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22384,"parent_record_offset":22352,"ancestor_record_offset":21948,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22480,"parent_record_offset":22384,"ancestor_record_offset":21948,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":21948,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22164,"parent_record_offset":22128,"ancestor_record_offset":21948,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":22504,"parent_record_offset":22480,"ancestor_record_offset":21948,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":22784,"parent_record_offset":22752,"ancestor_record_offset":21948,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":23068,"parent_record_offset":22784,"ancestor_record_offset":21948,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":23040,"parent_record_offset":22784,"ancestor_record_offset":21948,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":22844,"parent_record_offset":22784,"ancestor_record_offset":21948,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":23592,"parent_record_offset":23560,"ancestor_record_offset":21948,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":24660,"parent_record_offset":24628,"ancestor_record_offset":21948,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":25160,"parent_record_offset":24900,"ancestor_record_offset":21948,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":21948,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":23800,"parent_record_offset":23656,"ancestor_record_offset":21948,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":24060,"parent_record_offset":23800,"ancestor_record_offset":21948,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":23752,"parent_record_offset":23724,"ancestor_record_offset":21948,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":23656,"parent_record_offset":23624,"ancestor_record_offset":21948,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":23624,"parent_record_offset":23592,"ancestor_record_offset":21948,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::commit_scratch_to_composite<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":25752,"rva":10577280,"va":5379286400}
// 编译实例 RVAs: 0xa16580, 0x2262a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32382840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e8781fee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379287123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32560129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e801d4f001","size":5,"soft_fail":false,"target_va":5411847168,"va":5379287034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32254374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e8a629ec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379287205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32256068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e84430ec01","size":5,"soft_fail":false,"target_va":5411543264,"va":5379287191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-7155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e80de4ffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379286462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32256820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e83433ec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379286535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2098550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e876052000","size":5,"soft_fail":false,"target_va":5381385536,"va":5379286981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2103736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e8b8192000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379286899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2099386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e8ba082000","size":5,"soft_fail":false,"target_va":5381386480,"va":5379287089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2098269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e85d042000","size":5,"soft_fail":false,"target_va":5381384704,"va":5379286430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7042568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404764768,"raw_hex":"e808766b00","size":5,"soft_fail":false,"target_va":5411808064,"va":5404765491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9537971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e8b3899100","size":5,"soft_fail":false,"target_va":5388825296,"va":5379287320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32249930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379286400,"raw_hex":"e84a18ec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379287169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":28400,"parent_record_offset":25752,"ancestor_record_offset":25752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":28332,"parent_record_offset":25752,"ancestor_record_offset":25752,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":27364,"parent_record_offset":25752,"ancestor_record_offset":25752,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":25932,"parent_record_offset":25752,"ancestor_record_offset":25752,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":26388,"parent_record_offset":25752,"ancestor_record_offset":25752,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27684,"parent_record_offset":27656,"ancestor_record_offset":25752,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27736,"parent_record_offset":27712,"ancestor_record_offset":25752,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27712,"parent_record_offset":27684,"ancestor_record_offset":25752,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27632,"parent_record_offset":27604,"ancestor_record_offset":25752,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27656,"parent_record_offset":27632,"ancestor_record_offset":25752,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27828,"parent_record_offset":27656,"ancestor_record_offset":25752,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":25752,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":28368,"parent_record_offset":28332,"ancestor_record_offset":25752,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":27760,"parent_record_offset":27736,"ancestor_record_offset":25752,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":26088,"parent_record_offset":26056,"ancestor_record_offset":25752,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":27500,"parent_record_offset":27460,"ancestor_record_offset":25752,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":26336,"parent_record_offset":26308,"ancestor_record_offset":25752,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":28288,"parent_record_offset":27460,"ancestor_record_offset":25752,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":27892,"parent_record_offset":27604,"ancestor_record_offset":25752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":26800,"parent_record_offset":26776,"ancestor_record_offset":25752,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":26776,"parent_record_offset":26752,"ancestor_record_offset":25752,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":25752,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":27784,"parent_record_offset":27760,"ancestor_record_offset":25752,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":25752,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":26752,"parent_record_offset":26728,"ancestor_record_offset":25752,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":27056,"parent_record_offset":27028,"ancestor_record_offset":25752,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":26704,"parent_record_offset":26588,"ancestor_record_offset":25752,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":26056,"parent_record_offset":26028,"ancestor_record_offset":25752,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":26416,"parent_record_offset":26388,"ancestor_record_offset":25752,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":26556,"parent_record_offset":26524,"ancestor_record_offset":25752,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":26524,"parent_record_offset":26488,"ancestor_record_offset":25752,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":25752,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":26488,"parent_record_offset":26464,"ancestor_record_offset":25752,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":26440,"parent_record_offset":26416,"ancestor_record_offset":25752,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":27028,"parent_record_offset":26488,"ancestor_record_offset":25752,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":26028,"parent_record_offset":25996,"ancestor_record_offset":25752,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":25996,"parent_record_offset":25968,"ancestor_record_offset":25752,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":26188,"parent_record_offset":26156,"ancestor_record_offset":25752,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":26284,"parent_record_offset":26188,"ancestor_record_offset":25752,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":25752,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":25968,"parent_record_offset":25932,"ancestor_record_offset":25752,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":26308,"parent_record_offset":26284,"ancestor_record_offset":25752,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":26588,"parent_record_offset":26556,"ancestor_record_offset":25752,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":26872,"parent_record_offset":26588,"ancestor_record_offset":25752,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":26844,"parent_record_offset":26588,"ancestor_record_offset":25752,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":26648,"parent_record_offset":26588,"ancestor_record_offset":25752,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":27396,"parent_record_offset":27364,"ancestor_record_offset":25752,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":28464,"parent_record_offset":28432,"ancestor_record_offset":25752,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":28964,"parent_record_offset":28704,"ancestor_record_offset":25752,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":27528,"parent_record_offset":27500,"ancestor_record_offset":25752,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":27604,"parent_record_offset":27460,"ancestor_record_offset":25752,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":27864,"parent_record_offset":27604,"ancestor_record_offset":25752,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":27556,"parent_record_offset":27528,"ancestor_record_offset":25752,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":27460,"parent_record_offset":27428,"ancestor_record_offset":25752,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":27428,"parent_record_offset":27396,"ancestor_record_offset":25752,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 379
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A401
// 首条位置: {"module":3128,"inlinee":107521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3128,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4948,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/03128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A404
// 首条位置: {"module":3128,"inlinee":107524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3128,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":7568,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/03128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3FE
// 首条位置: {"module":3128,"inlinee":107518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$6::saturating_add
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3128,"record_offset":2176,"parent_record_offset":2148,"ancestor_record_offset":2148,"inlinee":91626,"source_line":1811,"archive":"_pdb/B/modules/03128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::extend_dirty
// 记录形式: inline_instance, inlinee_declaration; instances=54
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7835
// 首条位置: {"module":536,"inlinee":30773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=48
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":9916,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::copy_to_staging<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":41524,"rva":10580688,"va":5379289808}
// 编译实例 RVAs: 0xa172d0, 0x2261760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9133112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8c8a374ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5379291235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140adfcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t820945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8d1860c00","size":5,"soft_fail":false,"target_va":5380111536,"va":5379290586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141453350
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10730813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e83dbda300","size":5,"soft_fail":false,"target_va":5390021456,"va":5379290638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425da150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3639009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404759904,"raw_hex":"e8e1863700","size":5,"soft_fail":false,"target_va":5408399696,"va":5404760682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425db920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3645053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404759904,"raw_hex":"e87d9e3700","size":5,"soft_fail":false,"target_va":5408405792,"va":5404760734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32379104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8e010ee01","size":5,"soft_fail":false,"target_va":5411669968,"va":5379290859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32556536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8f8c5f001","size":5,"soft_fail":false,"target_va":5411847168,"va":5379290627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32557315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e803c9f001","size":5,"soft_fail":false,"target_va":5411847200,"va":5379289880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32250662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8261bec01","size":5,"soft_fail":false,"target_va":5411541584,"va":5379290917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-10621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e883d6ffff","size":5,"soft_fail":false,"target_va":5379279312,"va":5379289928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32253354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8aa25ec01","size":5,"soft_fail":false,"target_va":5411543360,"va":5379290001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32201432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8d85aeb01","size":5,"soft_fail":false,"target_va":5411492368,"va":5379290931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32203849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e84964eb01","size":5,"soft_fail":false,"target_va":5411494800,"va":5379290946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2532924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e83ca62600","size":5,"soft_fail":false,"target_va":5381823600,"va":5379290671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2533282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8a2a72600","size":5,"soft_fail":false,"target_va":5381824016,"va":5379290729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2095069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8ddf71f00","size":5,"soft_fail":false,"target_va":5381385536,"va":5379290462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2100258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8220c2000","size":5,"soft_fail":false,"target_va":5381390640,"va":5379290377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2095658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e82afa1f00","size":5,"soft_fail":false,"target_va":5381386480,"va":5379290817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2094800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8d0f61f00","size":5,"soft_fail":false,"target_va":5381384704,"va":5379289899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7047104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404759904,"raw_hex":"e8c0876b00","size":5,"soft_fail":false,"target_va":5411808064,"va":5404760955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9534187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8eb7a9100","size":5,"soft_fail":false,"target_va":5388825296,"va":5379291104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32246194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379289808,"raw_hex":"e8b209ec01","size":5,"soft_fail":false,"target_va":5411537104,"va":5379290905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":539,"record_offset":44252,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":44520,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":41708,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$4::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":539,"record_offset":44224,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":30989,"source_line":13,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":44280,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":44436,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":43256,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":41824,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":42280,"parent_record_offset":41524,"ancestor_record_offset":41524,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43576,"parent_record_offset":43548,"ancestor_record_offset":41524,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43628,"parent_record_offset":43604,"ancestor_record_offset":41524,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43604,"parent_record_offset":43576,"ancestor_record_offset":41524,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43524,"parent_record_offset":43496,"ancestor_record_offset":41524,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43548,"parent_record_offset":43524,"ancestor_record_offset":41524,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43720,"parent_record_offset":43548,"ancestor_record_offset":41524,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":539,"record_offset":43832,"parent_record_offset":43808,"ancestor_record_offset":41524,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":43652,"parent_record_offset":43628,"ancestor_record_offset":41524,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":539,"record_offset":41980,"parent_record_offset":41948,"ancestor_record_offset":41524,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":43392,"parent_record_offset":43352,"ancestor_record_offset":41524,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":42228,"parent_record_offset":42200,"ancestor_record_offset":41524,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":44180,"parent_record_offset":43352,"ancestor_record_offset":41524,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":43784,"parent_record_offset":43496,"ancestor_record_offset":41524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":42692,"parent_record_offset":42668,"ancestor_record_offset":41524,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":42668,"parent_record_offset":42644,"ancestor_record_offset":41524,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":41784,"parent_record_offset":41756,"ancestor_record_offset":41524,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":43676,"parent_record_offset":43652,"ancestor_record_offset":41524,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":42620,"parent_record_offset":42596,"ancestor_record_offset":41524,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":41524,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":42948,"parent_record_offset":42920,"ancestor_record_offset":41524,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":42596,"parent_record_offset":42480,"ancestor_record_offset":41524,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":41756,"parent_record_offset":41732,"ancestor_record_offset":41524,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":539,"record_offset":41732,"parent_record_offset":41708,"ancestor_record_offset":41524,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":41948,"parent_record_offset":41920,"ancestor_record_offset":41524,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":539,"record_offset":42308,"parent_record_offset":42280,"ancestor_record_offset":41524,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":42448,"parent_record_offset":42416,"ancestor_record_offset":41524,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":539,"record_offset":42416,"parent_record_offset":42380,"ancestor_record_offset":41524,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":41524,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":42380,"parent_record_offset":42356,"ancestor_record_offset":41524,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":42332,"parent_record_offset":42308,"ancestor_record_offset":41524,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":539,"record_offset":42920,"parent_record_offset":42380,"ancestor_record_offset":41524,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":41920,"parent_record_offset":41888,"ancestor_record_offset":41524,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":41888,"parent_record_offset":41860,"ancestor_record_offset":41524,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":42080,"parent_record_offset":42048,"ancestor_record_offset":41524,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":42176,"parent_record_offset":42080,"ancestor_record_offset":41524,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":41524,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":41860,"parent_record_offset":41824,"ancestor_record_offset":41524,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":42200,"parent_record_offset":42176,"ancestor_record_offset":41524,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":42480,"parent_record_offset":42448,"ancestor_record_offset":41524,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":42764,"parent_record_offset":42480,"ancestor_record_offset":41524,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":42736,"parent_record_offset":42480,"ancestor_record_offset":41524,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":539,"record_offset":42540,"parent_record_offset":42480,"ancestor_record_offset":41524,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":44640,"parent_record_offset":44576,"ancestor_record_offset":41524,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":44344,"parent_record_offset":44308,"ancestor_record_offset":41524,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":44664,"parent_record_offset":44640,"ancestor_record_offset":41524,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":539,"record_offset":44368,"parent_record_offset":44344,"ancestor_record_offset":41524,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":539,"record_offset":44548,"parent_record_offset":44520,"ancestor_record_offset":41524,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":539,"record_offset":44308,"parent_record_offset":44280,"ancestor_record_offset":41524,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":539,"record_offset":44576,"parent_record_offset":44548,"ancestor_record_offset":41524,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":539,"record_offset":44400,"parent_record_offset":44308,"ancestor_record_offset":41524,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":539,"record_offset":44612,"parent_record_offset":44576,"ancestor_record_offset":41524,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":43288,"parent_record_offset":43256,"ancestor_record_offset":41524,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":44772,"parent_record_offset":44740,"ancestor_record_offset":41524,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":45272,"parent_record_offset":45012,"ancestor_record_offset":41524,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":539,"record_offset":43420,"parent_record_offset":43392,"ancestor_record_offset":41524,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":43496,"parent_record_offset":43352,"ancestor_record_offset":41524,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":539,"record_offset":43756,"parent_record_offset":43496,"ancestor_record_offset":41524,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":539,"record_offset":43448,"parent_record_offset":43420,"ancestor_record_offset":41524,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":43352,"parent_record_offset":43320,"ancestor_record_offset":41524,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":539,"record_offset":43320,"parent_record_offset":43288,"ancestor_record_offset":41524,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::copy_to_staging::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 434
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F1D
// 首条位置: {"module":568,"inlinee":32541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::finish
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":568,"record_offset":71416,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":6343,"source_line":196,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":568,"record_offset":70984,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":6322,"source_line":171,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":568,"record_offset":71512,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":71676,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":71608,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::prepare_tile
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":568,"record_offset":69732,"parent_record_offset":69632,"ancestor_record_offset":69632,"inlinee":30997,"source_line":225,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":568,"record_offset":71140,"parent_record_offset":70984,"ancestor_record_offset":69632,"inlinee":6333,"source_line":110,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::large_update
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":568,"record_offset":71020,"parent_record_offset":70984,"ancestor_record_offset":69632,"inlinee":6326,"source_line":111,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::folded_multiply
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/operations.rs
//     evidence={"module":568,"record_offset":71048,"parent_record_offset":71020,"ancestor_record_offset":69632,"inlinee":6328,"source_line":17,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":568,"record_offset":70752,"parent_record_offset":70588,"ancestor_record_offset":69632,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":71076,"parent_record_offset":71048,"ancestor_record_offset":69632,"inlinee":6330,"source_line":2065,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":71108,"parent_record_offset":71020,"ancestor_record_offset":69632,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":568,"record_offset":71372,"parent_record_offset":71280,"ancestor_record_offset":69632,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":568,"record_offset":71236,"parent_record_offset":71212,"ancestor_record_offset":69632,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":70588,"parent_record_offset":69732,"ancestor_record_offset":69632,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":71164,"parent_record_offset":71140,"ancestor_record_offset":69632,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":71188,"parent_record_offset":71164,"ancestor_record_offset":69632,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":568,"record_offset":71212,"parent_record_offset":71188,"ancestor_record_offset":69632,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70032,"parent_record_offset":69732,"ancestor_record_offset":69632,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70924,"parent_record_offset":69732,"ancestor_record_offset":69632,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":71568,"parent_record_offset":71540,"ancestor_record_offset":69632,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70376,"parent_record_offset":70172,"ancestor_record_offset":69632,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70948,"parent_record_offset":70924,"ancestor_record_offset":69632,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":568,"record_offset":70096,"parent_record_offset":70032,"ancestor_record_offset":69632,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::on_uploaded_to_texture
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 547
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1698
// 首条位置: {"module":173,"inlinee":5784}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$2::on_texture_processed
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 556
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E8F
// 首条位置: {"module":502,"inlinee":28303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":502,"record_offset":76136,"parent_record_offset":76108,"ancestor_record_offset":76108,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::build_tile_hasher
// 记录形式: compiled_procedure; instances=1
// 源码行: 562
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":509,"record_offset":71964,"rva":9671648,"va":5378380768}
// 编译实例 RVAs: 0x9393e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::random_state::impl$5::build_hasher
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":509,"record_offset":72104,"parent_record_offset":71964,"ancestor_record_offset":71964,"inlinee":29014,"source_line":418,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::from_random_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":509,"record_offset":72124,"parent_record_offset":72104,"ancestor_record_offset":71964,"inlinee":29015,"source_line":57,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$4::load_index_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=80
// 源码行: 583
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F4
// 首条位置: {"module":539,"inlinee":30964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::impl$45::deref_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 657
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":78464,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::impl$69::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 657
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":78604,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone1Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x837C
// 首条位置: {"module":596,"inlinee":33660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone2Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x837E
// 首条位置: {"module":596,"inlinee":33662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone3Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8380
// 首条位置: {"module":596,"inlinee":33664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::GardenRaster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x838A
// 首条位置: {"module":597,"inlinee":33674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::PathRaster::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 665
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6675
// 首条位置: {"module":476,"record_offset":69408,"rva":8645360,"va":5377354480}
// 编译实例 RVAs: 0x83eaf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1927731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354480,"raw_hex":"e8336a1d00","size":5,"soft_fail":false,"target_va":5379282224,"va":5377354488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 678
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":78884,"rva":244608,"va":5368953728}
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
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::get_param
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 678, 695
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x370E
// 首条位置: {"module":178,"inlinee":14094}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$107::get_param
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":178,"record_offset":2912360,"parent_record_offset":2912324,"ancestor_record_offset":2912324,"inlinee":4315,"source_line":2106,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":2912804,"parent_record_offset":2912780,"ancestor_record_offset":2912324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":2912852,"parent_record_offset":2912828,"ancestor_record_offset":2912324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":2912828,"parent_record_offset":2912804,"ancestor_record_offset":2912324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":2912780,"parent_record_offset":2912756,"ancestor_record_offset":2912324,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":2912756,"parent_record_offset":2912716,"ancestor_record_offset":2912324,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":2912716,"parent_record_offset":2912692,"ancestor_record_offset":2912324,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":178,"record_offset":2913120,"parent_record_offset":2913096,"ancestor_record_offset":2912324,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":178,"record_offset":2913096,"parent_record_offset":2913064,"ancestor_record_offset":2912324,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":178,"record_offset":2913036,"parent_record_offset":2913000,"ancestor_record_offset":2912324,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":2912972,"parent_record_offset":2912432,"ancestor_record_offset":2912324,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":2913000,"parent_record_offset":2912972,"ancestor_record_offset":2912324,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":2912472,"parent_record_offset":2912432,"ancestor_record_offset":2912324,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":2913064,"parent_record_offset":2913000,"ancestor_record_offset":2912324,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":2912520,"parent_record_offset":2912496,"ancestor_record_offset":2912324,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":2912496,"parent_record_offset":2912472,"ancestor_record_offset":2912324,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":2912692,"parent_record_offset":2912668,"ancestor_record_offset":2912324,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":178,"record_offset":2912396,"parent_record_offset":2912360,"ancestor_record_offset":2912324,"inlinee":4173,"source_line":1020,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param::closure$0
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":178,"record_offset":2913304,"parent_record_offset":2913280,"ancestor_record_offset":2912324,"inlinee":4208,"source_line":1028,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$9::get_param
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2334488,"parent_record_offset":2334448,"ancestor_record_offset":2334408,"inlinee":4134,"source_line":905,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$9::get_param::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2335068,"parent_record_offset":2335044,"ancestor_record_offset":2334408,"inlinee":4199,"source_line":914,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":178,"record_offset":2913188,"parent_record_offset":2913164,"ancestor_record_offset":2912324,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":178,"record_offset":2913164,"parent_record_offset":2912432,"ancestor_record_offset":2912324,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":178,"record_offset":2912432,"parent_record_offset":2912396,"ancestor_record_offset":2912324,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1611,"record_offset":2334524,"parent_record_offset":2334488,"ancestor_record_offset":2334408,"inlinee":4135,"source_line":616,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":178,"record_offset":2913420,"parent_record_offset":2913304,"ancestor_record_offset":2912324,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":178,"record_offset":2913360,"parent_record_offset":2913336,"ancestor_record_offset":2912324,"inlinee":4202,"source_line":99,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new_display
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":178,"record_offset":2913336,"parent_record_offset":2913304,"ancestor_record_offset":2912324,"inlinee":4201,"source_line":113,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":178,"record_offset":2912608,"parent_record_offset":2912548,"ancestor_record_offset":2912324,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":2912628,"parent_record_offset":2912608,"ancestor_record_offset":2912324,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":2912668,"parent_record_offset":2912496,"ancestor_record_offset":2912324,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":2913280,"parent_record_offset":2912396,"ancestor_record_offset":2912324,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::from_ref
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":178,"record_offset":2913384,"parent_record_offset":2913360,"ancestor_record_offset":2912324,"inlinee":4204,"source_line":268,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":178,"record_offset":2912548,"parent_record_offset":2912520,"ancestor_record_offset":2912324,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":2912920,"parent_record_offset":2912896,"ancestor_record_offset":2912324,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":2912580,"parent_record_offset":2912548,"ancestor_record_offset":2912324,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":2912896,"parent_record_offset":2912716,"ancestor_record_offset":2912324,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":178,"record_offset":2913212,"parent_record_offset":2913188,"ancestor_record_offset":2912324,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":178,"record_offset":2913236,"parent_record_offset":2913212,"ancestor_record_offset":2912324,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1611,"record_offset":2334800,"parent_record_offset":2334776,"ancestor_record_offset":2334408,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::init_state
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 678, 695
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6685
// 首条位置: {"module":476,"record_offset":79156,"rva":8650272,"va":5377359392}
// 编译实例 RVAs: 0x83fe20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..change_detection..ResMut$LT$T$GT$$u20$as$u20$bevy_ecs..system..system_param..SystemParam$GT$::init_state
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t11741132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377359392,"raw_hex":"e8cc27b300","size":5,"soft_fail":false,"target_va":5389100560,"va":5377359423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$107::init_state
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":476,"record_offset":79308,"parent_record_offset":79156,"ancestor_record_offset":79156,"inlinee":4310,"source_line":2064,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::new_archetype
// 记录形式: compiled_procedure; instances=1
// 源码行: 678
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":78740,"rva":244608,"va":5368953728}
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
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::queue
// 记录形式: compiled_procedure; instances=1
// 源码行: 678
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":79020,"rva":244608,"va":5368953728}
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
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::validate_param
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 678, 695
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3708
// 首条位置: {"module":178,"inlinee":14088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$12::validate_param
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":178,"record_offset":890036,"parent_record_offset":890012,"ancestor_record_offset":890012,"inlinee":4225,"source_line":1000,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$9::validate_param
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":1301780,"parent_record_offset":1301756,"ancestor_record_offset":1301756,"inlinee":4223,"source_line":885,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":890388,"parent_record_offset":890364,"ancestor_record_offset":890012,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":890436,"parent_record_offset":890412,"ancestor_record_offset":890012,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":890412,"parent_record_offset":890388,"ancestor_record_offset":890012,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":890364,"parent_record_offset":890340,"ancestor_record_offset":890012,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":890340,"parent_record_offset":890308,"ancestor_record_offset":890012,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":890308,"parent_record_offset":890284,"ancestor_record_offset":890012,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":178,"record_offset":890568,"parent_record_offset":890544,"ancestor_record_offset":890012,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":890544,"parent_record_offset":890524,"ancestor_record_offset":890012,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":178,"record_offset":890064,"parent_record_offset":890036,"ancestor_record_offset":890012,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":890112,"parent_record_offset":890088,"ancestor_record_offset":890012,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":890088,"parent_record_offset":890064,"ancestor_record_offset":890012,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":178,"record_offset":890284,"parent_record_offset":890260,"ancestor_record_offset":890012,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":178,"record_offset":890200,"parent_record_offset":890140,"ancestor_record_offset":890012,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":890220,"parent_record_offset":890200,"ancestor_record_offset":890012,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":890500,"parent_record_offset":890036,"ancestor_record_offset":890012,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":178,"record_offset":890260,"parent_record_offset":890088,"ancestor_record_offset":890012,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":178,"record_offset":890140,"parent_record_offset":890112,"ancestor_record_offset":890012,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":178,"record_offset":890172,"parent_record_offset":890140,"ancestor_record_offset":890012,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":178,"record_offset":890888,"parent_record_offset":890864,"ancestor_record_offset":890012,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00178.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6677
// 首条位置: {"module":1722,"inlinee":26231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::last_staged
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1722,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":12112,"inlinee":26191,"source_line":192,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names
// 记录形式: compiled_procedure; instances=1
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":69544,"rva":8645392,"va":5377354512}
// 编译实例 RVAs: 0x83eb10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2273054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354512,"raw_hex":"e8e250ddff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377354537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34186285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354512,"raw_hex":"e82da40902","size":5,"soft_fail":false,"target_va":5411540992,"va":5377354702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":69712,"parent_record_offset":69544,"ancestor_record_offset":69544,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":69852,"parent_record_offset":69544,"ancestor_record_offset":69544,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":69760,"parent_record_offset":69740,"ancestor_record_offset":69544,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":69784,"parent_record_offset":69760,"ancestor_record_offset":69544,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":69740,"parent_record_offset":69712,"ancestor_record_offset":69544,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":69876,"parent_record_offset":69852,"ancestor_record_offset":69544,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":69900,"parent_record_offset":69876,"ancestor_record_offset":69544,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":69808,"parent_record_offset":69784,"ancestor_record_offset":69544,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::write
// 记录形式: compiled_procedure; instances=1
// 源码行: 712
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":69940,"rva":8645600,"va":5377354720}
// 编译实例 RVAs: 0x83ebe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15140
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1925543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e8a7611d00","size":5,"soft_fail":false,"target_va":5379281216,"va":5377355668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33985599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e83f940602","size":5,"soft_fail":false,"target_va":5411341280,"va":5377355676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34492317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e89d4f0e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377354846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34492393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e8e94f0e02","size":5,"soft_fail":false,"target_va":5411847200,"va":5377354802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5298465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e821d95000","size":5,"soft_fail":false,"target_va":5382655024,"va":5377356554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34185262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e82ea00902","size":5,"soft_fail":false,"target_va":5411541776,"va":5377356509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34187155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377354720,"raw_hex":"e893a70902","size":5,"soft_fail":false,"target_va":5411542880,"va":5377355720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":70176,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":74136,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":476,"record_offset":73752,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":70116,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::clear_composite
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":476,"record_offset":70232,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":26192,"source_line":214,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::replace_composite
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":476,"record_offset":70584,"parent_record_offset":69940,"ancestor_record_offset":69940,"inlinee":26193,"source_line":199,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70364,"parent_record_offset":70340,"ancestor_record_offset":69940,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70412,"parent_record_offset":70388,"ancestor_record_offset":69940,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":70388,"parent_record_offset":70364,"ancestor_record_offset":69940,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70340,"parent_record_offset":70312,"ancestor_record_offset":69940,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70312,"parent_record_offset":70284,"ancestor_record_offset":69940,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70256,"parent_record_offset":70232,"ancestor_record_offset":69940,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":70284,"parent_record_offset":70256,"ancestor_record_offset":69940,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":70200,"parent_record_offset":70176,"ancestor_record_offset":69940,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":476,"record_offset":70680,"parent_record_offset":70584,"ancestor_record_offset":69940,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":70648,"parent_record_offset":70624,"ancestor_record_offset":69940,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":73784,"parent_record_offset":73752,"ancestor_record_offset":69940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":70144,"parent_record_offset":70116,"ancestor_record_offset":69940,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":70624,"parent_record_offset":70584,"ancestor_record_offset":69940,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":70464,"parent_record_offset":70232,"ancestor_record_offset":69940,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed
// 记录形式: compiled_procedure; instances=1
// 源码行: 726
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":74168,"rva":8647696,"va":5377356816}
// 编译实例 RVAs: 0x83f410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33983549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356816,"raw_hex":"e83d8c0602","size":5,"soft_fail":false,"target_va":5411341280,"va":5377357726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356816,"raw_hex":"e8edc6ffff","size":5,"soft_fail":false,"target_va":5377343808,"va":5377358414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5482485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356816,"raw_hex":"e8f5a75300","size":5,"soft_fail":false,"target_va":5382840208,"va":5377357718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":76044,"parent_record_offset":74168,"ancestor_record_offset":74168,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":74408,"parent_record_offset":74168,"ancestor_record_offset":74168,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":476,"record_offset":76128,"parent_record_offset":74168,"ancestor_record_offset":74168,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":74352,"parent_record_offset":74168,"ancestor_record_offset":74168,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::last_staged
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":476,"record_offset":74600,"parent_record_offset":74168,"ancestor_record_offset":74168,"inlinee":26191,"source_line":192,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":76092,"parent_record_offset":76068,"ancestor_record_offset":74168,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":74436,"parent_record_offset":74408,"ancestor_record_offset":74168,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":74492,"parent_record_offset":74464,"ancestor_record_offset":74168,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":76068,"parent_record_offset":76044,"ancestor_record_offset":74168,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":74380,"parent_record_offset":74352,"ancestor_record_offset":74168,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":74520,"parent_record_offset":74492,"ancestor_record_offset":74168,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":74548,"parent_record_offset":74520,"ancestor_record_offset":74168,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":74464,"parent_record_offset":74436,"ancestor_record_offset":74168,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 记录形式: compiled_procedure; instances=1
// 源码行: 753
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":76216,"rva":8649312,"va":5377358432}
// 编译实例 RVAs: 0x83fa60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1219734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377358432,"raw_hex":"e8969c1200","size":5,"soft_fail":false,"target_va":5378578208,"va":5377358469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1931224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377358432,"raw_hex":"e8d8771d00","size":5,"soft_fail":false,"target_va":5379289808,"va":5377358579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34488612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377358432,"raw_hex":"e824410e02","size":5,"soft_fail":false,"target_va":5411847152,"va":5377358535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34182385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377358432,"raw_hex":"e8f1940902","size":5,"soft_fail":false,"target_va":5411541776,"va":5377359386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34123099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377358432,"raw_hex":"e85bad0802","size":5,"soft_fail":false,"target_va":5411482433,"va":5377359329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1930519","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377358432,"raw_hex":"e917751d00","size":5,"soft_fail":false,"target_va":5379289808,"va":5377359284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5377358432,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5377358555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":76396,"parent_record_offset":76216,"ancestor_record_offset":76216,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":76672,"parent_record_offset":76216,"ancestor_record_offset":76216,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":476,"record_offset":76448,"parent_record_offset":76216,"ancestor_record_offset":76216,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":476,"record_offset":76700,"parent_record_offset":76216,"ancestor_record_offset":76216,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":76420,"parent_record_offset":76396,"ancestor_record_offset":76216,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":76504,"parent_record_offset":76472,"ancestor_record_offset":76216,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":76588,"parent_record_offset":76560,"ancestor_record_offset":76216,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":76616,"parent_record_offset":76588,"ancestor_record_offset":76216,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":76532,"parent_record_offset":76504,"ancestor_record_offset":76216,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":476,"record_offset":76472,"parent_record_offset":76448,"ancestor_record_offset":76216,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 完整结构及原始类型记录: world_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::DirtyTilePublisher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::world_raster::DirtyTilePublisher>", "constructor": "enum2$", "arguments": ["country_core::resources::world_raster::DirtyTilePublisher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::world_raster::DirtyTilePublisher> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::world_raster::DirtyTilePublisher>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::RasterBufferRef
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::RasterBufferRef<512>", "constructor": "country_core::resources::world_raster::RasterBufferRef", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >", "constructor": "utils::float_grid::impl$0::bilinear_sample_at_world_xz", "arguments": ["country_core::resources::world_raster::RasterBufferRef<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::StrokeBlendMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::world_raster::StrokeBlendMode>", "constructor": "enum2$", "arguments": ["country_core::resources::world_raster::StrokeBlendMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 35
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::collections::Events", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$9::init_state<bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$9::init_state", "arguments": ["bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::reader::EventReader", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::registry::EventRegistry::register_event", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::base::EventInstance", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["bevy_ecs::event::base::EventInstance<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$12::init_state<bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$12::init_state", "arguments": ["bevy_ecs::event::collections::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::WorldRaster
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster<512>", "constructor": "country_core::resources::world_raster::WorldRaster", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::world_raster::WorldRaster<512> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::world_raster::WorldRaster<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster::new<512>", "constructor": "country_core::resources::world_raster::WorldRaster::new", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster::subscribe<512>", "constructor": "country_core::resources::world_raster::WorldRaster::subscribe", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$6", "country_core::systems::paths::draw_path::draw_path::closure_env$7", "country_core::systems::paths::draw_path::draw_path::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$12", "country_core::systems::paths::draw_path::draw_path::closure_env$13", "country_core::systems::paths::draw_path::draw_path::closure_env$14"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::garden::draw_garden::draw_garden::closure_env$0", "country_core::systems::garden::draw_garden::draw_garden::closure_env$1", "country_core::systems::garden::draw_garden::draw_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$9", "country_core::systems::paths::draw_path::draw_path::closure_env$10", "country_core::systems::paths::draw_path::draw_path::closure_env$11"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$0", "country_core::systems::paths::draw_path::draw_path::closure_env$1", "country_core::systems::paths::draw_path::draw_path::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$3", "country_core::systems::paths::draw_path::draw_path::closure_env$4", "country_core::systems::paths::draw_path::draw_path::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > >", "constructor": "country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >", "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >", "constructor": "conservative_line_raster::conservative_plot_line", "arguments": ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "system_water::draw_water::draw_water::closure_env$1", "system_water::draw_water::draw_water::closure_env$2", "system_water::draw_water::draw_water::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: world_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::world_raster::RasterBufferRef", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::world_raster::RasterBufferRef", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 495; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "country_core::systems::save::snapshot::SnapshotLoadResources", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >", "tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::secondary
// 信息 ID: 465d702f8727a17b4704f8a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::shared
// 信息 ID: 1457eb7789487d17ccce394b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::unique
// 信息 ID: 5b50acbd582471e00a7250eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_water::impl$0::build @ 0x650360 -> 0x9393b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=597, inline_record=131320; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::as_float_grid
// 信息 ID: aa4b95715ffc0a846cb6073d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 46 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=443, inline_record=15000; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs; module=443, inline_record=21904; 不是独立调用边。
// [PDB 内联宿主] system_decorator::audio::op_stairs_audio::op_stairs_audio; module=467, inline_record=10972; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain; module=524, inline_record=14476; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_system; module=531, inline_record=60524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=556, inline_record=17268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=556, inline_record=17336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=556, inline_record=17364; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::clear_composite
// 信息 ID: d82fd57ccb945edc1f166e3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=70232; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=70832; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=71440; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=72048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=72656; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=73264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::last_staged
// 信息 ID: 8f1a44452dbcfa8c0bd49631
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=74600; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=74880; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=75164; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=75448; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=75732; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=476, inline_record=76016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot_save_data; module=1722, inline_record=12136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot_save_data; module=1722, inline_record=12164; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::new<512>
// 信息 ID: 90e8022733af6870aec5e826
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::WorldRaster::new -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster_defs::PathRaster::new @ 0x83eaf8 -> 0xa15530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0xb69097 -> 0xa15530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0xb690dc -> 0xa15530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0xb69121 -> 0xa15530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0xb69186 -> 0xa15530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::impl$0::build @ 0xb6d3c0 -> 0xa15530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::prepare_tile
// 信息 ID: c5209474a0dc3dcbbf5ac1a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; module=568, inline_record=69732; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; module=4422, inline_record=2460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::replace_composite
// 信息 ID: f4292fb7ce29abae5daf8b10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=70584; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=71192; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=71800; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=72408; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=73016; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=476, inline_record=73624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::subscribe<512>
// 信息 ID: 3e07f7ca7c4d9a1dbf937819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc68acc99a848e3e9f4a896d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster::subscribe -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x8c630a -> 0xa158e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > > @ 0x14d7626 -> 0xa158e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x22d6c03 -> 0x2612be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x22d6c5a -> 0x2612be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x22d6cb1 -> 0x2612be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::ws_to_px
// 信息 ID: b8d69489181aa0179bcbec93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=539, inline_record=32948; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=539, inline_record=33532; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=539, inline_record=38932; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=539, inline_record=39516; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=3648, inline_record=7992; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=3648, inline_record=8576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: a440826201c3da942714bdd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1630, inline_record=494668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 953ffa73ac6b54231836f540
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1630, inline_record=494548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 84d7fb9a4114466c3f915226
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=539, inline_record=71336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 40aacf04729a24e101f4e6e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=654140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 7d5281c3cee03145b17fca0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=654016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 9ae76375e41cc8d23e20b6fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1630, inline_record=652488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 4dd73f7e5cae746c0134ff34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor> @ 0x15b047b -> 0xa197c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 961ffd6f702cbc638c4303e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x4c0898 -> 0x3c3b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x1588ef8 -> 0xa195a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::build_tile_hasher
// 信息 ID: 804cf229ecd963cfabcea76e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> > @ 0x25da382 -> 0x9393e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$14::fmt
// 信息 ID: 1327821ddff227224b95fbf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >; module=539, inline_record=1884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::clear_all<512>
// 信息 ID: 1d759718830aa160209d54db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$2::clear_all -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b1c8 -> 0xa15da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b1ef -> 0xa15da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b285 -> 0xa15da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b29a -> 0xa15da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b2af -> 0xa15da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b2c4 -> 0xa15da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::clear_buffer<512>
// 信息 ID: 8d97cbd04b986a239299defc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$2::clear_buffer -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f1e75 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f47d9 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f7175 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9f9ab5 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fc415 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0x9fed65 -> 0xa15a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::commit_scratch_to_composite<512>
// 信息 ID: 1a8557b8e4336bc6d017fd9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$2::commit_scratch_to_composite -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f3761 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f6124 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f8a63 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fb3a1 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fdd01 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xa00667 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8> @ 0xab14c3 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab288f -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> > @ 0xab3af7 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14> @ 0xab4cb2 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xab5f93 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab732f -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab85df -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11> @ 0xab98f3 -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xabacdc -> 0xa16580 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5> @ 0xabc033 -> 0xa16580 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::copy_composite_to_scratch<512>
// 信息 ID: 7c973a477c5185296723b15d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$2::copy_composite_to_scratch -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f35de -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f5f3c -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f88de -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fb21e -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fdb7e -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xa004e2 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8> @ 0xab2114 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab34c2 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> > @ 0xab4641 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14> @ 0xab5853 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xab6be4 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab7f62 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xab9212 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11> @ 0xaba544 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xabb873 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5> @ 0xabcc84 -> 0xa161a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::copy_to_staging::closure$0
// 信息 ID: e583db689986d20fd37d3a71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$2::copy_to_staging", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; module=568, inline_record=69632; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >; module=4422, inline_record=2356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::copy_to_staging<512>
// 信息 ID: d0f10d73642d67011f94928e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$2::copy_to_staging -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83faf3 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83fb7d -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83fc07 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83fc91 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83fd24 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x83fdb4 -> 0xa172d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f24b0 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f36a1 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f4e02 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f5fff -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f77a7 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f89a1 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fa0f0 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fb2e1 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fca50 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fdc41 -> 0xa172d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::extend_dirty
// 信息 ID: 0ef4a49efc84263502f9177d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9916; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=17848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=17904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=33588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=33644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=41872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=49832; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::on_texture_processed
// 信息 ID: 4f393a476188c30ab7b61685
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=76108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=76224; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=76340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=502, inline_record=76456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=538, inline_record=197536; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=540, inline_record=106736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::on_uploaded_to_texture
// 信息 ID: 6dc5bf456f039eca9bd5358c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=173, inline_record=1528; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>; module=173, inline_record=3652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=173, inline_record=5768; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=173, inline_record=7884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>; module=173, inline_record=10008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>; module=173, inline_record=12132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$0
// 信息 ID: aac12f793b278558ae2bb6db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$2::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=4948; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=5336; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=5532; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=12988; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=13376; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=13572; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=26296; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=26684; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$1
// 信息 ID: d5bad042d956a7f0839cfa67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::world_raster::impl$2::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=7568; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=7956; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=8156; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=10192; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=10580; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=10780; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=20880; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=21268; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::rasterize_stroke::closure$2
// 信息 ID: e07ee17857523651203f5ef0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::world_raster::impl$2::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=2148; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=2540; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=2736; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=15616; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=16008; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=16204; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=18252; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$2::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=3128, inline_record=18644; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 信息 ID: 7e21b3d81fb12ce551dfe298
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::impl$2::rasterize_stroke -> ["512", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f1fd8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f2058 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f361c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f4938 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f49b9 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f5f7a -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f72d8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f7358 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f891c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9f9c18 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9f9c98 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fb25c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fc578 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fc5f8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fdbbc -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0x9feec8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$2::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 信息 ID: 96606a4f4c9f67b5e01f63bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::impl$2::rasterize_stroke -> ["512", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f1fd8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f2058 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f361c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f4938 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f49b9 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f5f7a -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f72d8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f7358 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f891c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9f9c18 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9f9c98 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fb25c -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fc578 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fc5f8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fdbbc -> 0xa16960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0x9feec8 -> 0xa16960 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$4::load_index_unchecked
// 信息 ID: a628a2495ade4f25ecf19512
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 72 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=7188; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=7844; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=7916; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=7988; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=8056; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=8468; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=8544; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=539, inline_record=8616; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 信息 ID: db0c5b10db19f2a0e948cc32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot_save_data; module=1722, inline_record=12112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names
// 信息 ID: 5efe2750fb7edf302b0a9552
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x17a863c -> 0x83eb10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 信息 ID: d0b310546a68652263442dfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::upload_world_raster_buffers_to_staging @ 0x957eb8 -> 0x83fa60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed
// 信息 ID: 9b1ac957696f4de7265044a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x17a881f -> 0x83f410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::write
// 信息 ID: 84ea97d9a8fb8b083a7f8883
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::init_game_state_from_snapshot @ 0x17aa796 -> 0x83ebe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone1Raster::new
// 信息 ID: 6a78775372874dd6fdd0bdad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone2Raster::new
// 信息 ID: a624803ba6f7ba8ff3273434
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone3Raster::new
// 信息 ID: 54472bc7ec5c61dcd6b09013
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::GardenRaster::new
// 信息 ID: 7d9c8f7c58f7f2b444f734bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=597, inline_record=131376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::PathRaster::new
// 信息 ID: 20e86f0b2d938d50b29ce332
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_water::impl$0::build @ 0x65036f -> 0x83eaf0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=596, inline_record=108912; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::apply
// 信息 ID: 0c4bef0e66803a6e4e09e9d4
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
// 函数: country_core::resources::world_raster_defs::_::impl$0::get_param
// 信息 ID: 3686afe4856d15def64eaca3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c86bc3a5184ebadda09ca71: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 0cf452bde58780bc66d5e0be: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// [泛型宿主/闭包/直接引用方参数候选] de7e9a093f35917ccdf22ec8: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >; module=178, inline_record=2912324; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >; module=247, inline_record=5884; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> > >; module=1611, inline_record=2334408; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >; module=1611, inline_record=2673084; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >; module=1698, inline_record=20984; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> >; module=1699, inline_record=25244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::init_state
// 信息 ID: a36f3ff01265f7f190983d88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c86bc3a5184ebadda09ca71: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] de7e9a093f35917ccdf22ec8: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > @ 0x5beca3 -> 0x83fe20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> >; module=1699, inline_record=6700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::_::impl$0::init_state; module=1722, inline_record=107876; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::new_archetype
// 信息 ID: 63ca19c47415a3bdbfd0e581
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
// 函数: country_core::resources::world_raster_defs::_::impl$0::queue
// 信息 ID: bd32c8f564144d5d39f5bb0e
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
// 函数: country_core::resources::world_raster_defs::_::impl$0::validate_param
// 信息 ID: db9a1dd15ac169619c2c4ca8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c86bc3a5184ebadda09ca71: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 0cf452bde58780bc66d5e0be: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// [泛型宿主/闭包/直接引用方参数候选] de7e9a093f35917ccdf22ec8: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >; module=178, inline_record=890012; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >; module=247, inline_record=25632; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> > >; module=1611, inline_record=1301756; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >; module=1611, inline_record=1392528; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >; module=1698, inline_record=99776; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> >; module=1699, inline_record=99268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::impl$45::deref_mut
// 信息 ID: 1944f1e23e99cb609b828714
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::impl$69::deref
// 信息 ID: f77ddc2294500f406f9705cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
