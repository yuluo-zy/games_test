// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/utils/trim_rows.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3bf0f8ebd9d0630d4f6be7df1c1abb79f2b563b719488195fd531368eb7a136c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: trim_rows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1415, 1461, 3034
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::trim_rows
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":2052,"rva":19132144,"va":5387841264}
// 编译实例 RVAs: 0x123eef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12761284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e83c473dff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387842767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12761846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e80a453dff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387843345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe0750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14292244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e81415da00","size":5,"soft_fail":false,"target_va":5402134352,"va":5387842103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24005838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8ce4c6e01","size":5,"soft_fail":false,"target_va":5411848096,"va":5387842253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11530746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8060e50ff","size":5,"soft_fail":false,"target_va":5376312528,"va":5387843269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23697336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8b8976901","size":5,"soft_fail":false,"target_va":5411540963,"va":5387843622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23697977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8399a6901","size":5,"soft_fail":false,"target_va":5411541584,"va":5387843602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e862fbffff","size":5,"soft_fail":false,"target_va":5387840192,"va":5387841369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23701909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e895a96901","size":5,"soft_fail":false,"target_va":5411543360,"va":5387841446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6456344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8e87b9dff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387841875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6451158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e82a909dff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387841793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6456936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e898799dff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387843411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6456648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8b87a9dff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387841347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23798058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e82a216b01","size":5,"soft_fail":false,"target_va":5411640768,"va":5387842705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-279782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e81abbfbff","size":5,"soft_fail":false,"target_va":5387564432,"va":5387844209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23693509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387841264,"raw_hex":"e8c5886901","size":5,"soft_fail":false,"target_va":5411537104,"va":5387843590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":7140,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4672,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":6432,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5616,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":7686,"source_line":2096,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":6044,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1461,"record_offset":5400,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1461,"record_offset":6320,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1461,"record_offset":6620,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1461,"record_offset":8612,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":6864,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":7320,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":4700,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":5284,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1461,"record_offset":4784,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":3688,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":2256,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":2712,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1461,"record_offset":5372,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":2052,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":2052,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":2052,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":7484,"parent_record_offset":7392,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6500,"parent_record_offset":6468,"ancestor_record_offset":2052,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6124,"parent_record_offset":6084,"ancestor_record_offset":2052,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":2052,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":2052,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":2052,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":2052,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":6468,"parent_record_offset":6432,"ancestor_record_offset":2052,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":7204,"parent_record_offset":7140,"ancestor_record_offset":2052,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":2052,"inlinee":7687,"source_line":2122,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":2052,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":6084,"parent_record_offset":6044,"ancestor_record_offset":2052,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":8748,"parent_record_offset":8640,"ancestor_record_offset":2052,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":2052,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5716,"parent_record_offset":5684,"ancestor_record_offset":2052,"inlinee":7691,"source_line":2100,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":2052,"inlinee":7696,"source_line":2156,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5684,"parent_record_offset":5644,"ancestor_record_offset":2052,"inlinee":7689,"source_line":2176,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":2052,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":2052,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":4052,"parent_record_offset":4024,"ancestor_record_offset":2052,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":2052,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":4168,"parent_record_offset":3996,"ancestor_record_offset":2052,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1461,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":2052,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":6928,"parent_record_offset":6892,"ancestor_record_offset":2052,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":2052,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1461,"record_offset":5892,"parent_record_offset":5792,"ancestor_record_offset":2052,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1461,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":2052,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1461,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":2052,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1461,"record_offset":6752,"parent_record_offset":6620,"ancestor_record_offset":2052,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1461,"record_offset":6660,"parent_record_offset":6620,"ancestor_record_offset":2052,"inlinee":25349,"source_line":60,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1461,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":2052,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1461,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":2052,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1461,"record_offset":5924,"parent_record_offset":5644,"ancestor_record_offset":2052,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1461,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2052,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1461,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1461,"record_offset":2412,"parent_record_offset":2380,"ancestor_record_offset":2052,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1461,"record_offset":3840,"parent_record_offset":3796,"ancestor_record_offset":2052,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":2052,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":8928,"parent_record_offset":8748,"ancestor_record_offset":2052,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":2052,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":6720,"parent_record_offset":6660,"ancestor_record_offset":2052,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1461,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":2052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1461,"record_offset":4628,"parent_record_offset":3796,"ancestor_record_offset":2052,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":4232,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":2052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":2052,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2052,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2052,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":2052,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":2052,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1461,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":2052,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2052,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":2052,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2052,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":2052,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":4824,"parent_record_offset":4784,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":2052,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":3028,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":4900,"parent_record_offset":4824,"ancestor_record_offset":2052,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":8672,"parent_record_offset":8640,"ancestor_record_offset":2052,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1461,"record_offset":2380,"parent_record_offset":2352,"ancestor_record_offset":2052,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1461,"record_offset":2740,"parent_record_offset":2712,"ancestor_record_offset":2052,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1461,"record_offset":2880,"parent_record_offset":2848,"ancestor_record_offset":2052,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1461,"record_offset":2848,"parent_record_offset":2812,"ancestor_record_offset":2052,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1461,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2052,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1461,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2052,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1461,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2052,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1461,"record_offset":3352,"parent_record_offset":2812,"ancestor_record_offset":2052,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2352,"parent_record_offset":2320,"ancestor_record_offset":2052,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":2052,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":2052,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2608,"parent_record_offset":2512,"ancestor_record_offset":2052,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":2052,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2292,"parent_record_offset":2256,"ancestor_record_offset":2052,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1461,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":2052,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1461,"record_offset":2912,"parent_record_offset":2880,"ancestor_record_offset":2052,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1461,"record_offset":3196,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1461,"record_offset":3168,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1461,"record_offset":2972,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1461,"record_offset":4988,"parent_record_offset":4784,"ancestor_record_offset":2052,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":3724,"parent_record_offset":3688,"ancestor_record_offset":2052,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":7620,"parent_record_offset":7588,"ancestor_record_offset":2052,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":8136,"parent_record_offset":7876,"ancestor_record_offset":2052,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1461,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":2052,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":3944,"parent_record_offset":3796,"ancestor_record_offset":2052,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1461,"record_offset":4204,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1461,"record_offset":3896,"parent_record_offset":3868,"ancestor_record_offset":2052,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1461,"record_offset":3796,"parent_record_offset":3760,"ancestor_record_offset":2052,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1461,"record_offset":3760,"parent_record_offset":3724,"ancestor_record_offset":2052,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::trim_rows::trim_rows::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/trim_rows.rs
//     evidence={"module":1461,"record_offset":5748,"parent_record_offset":5716,"ancestor_record_offset":2052,"inlinee":70755,"source_line":88,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::trim_rows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11463
// 首条位置: {"module":1415,"inlinee":70755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::above
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x115D5
// 首条位置: {"module":3034,"inlinee":71125}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3034,"record_offset":648,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3034,"record_offset":860,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3034,"record_offset":832,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3034,"record_offset":532,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":31919,"source_line":157,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3034,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":484,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3034,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":484,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3034,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":484,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3034,"record_offset":680,"parent_record_offset":648,"ancestor_record_offset":484,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3034,"record_offset":612,"parent_record_offset":532,"ancestor_record_offset":484,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3034,"record_offset":800,"parent_record_offset":648,"ancestor_record_offset":484,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3034,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":484,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3034,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":484,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03034.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::above_to_below
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":9612,"rva":19135152,"va":5387844272}
// 编译实例 RVAs: 0x123fab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23697321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844272,"raw_hex":"e8a9976901","size":5,"soft_fail":false,"target_va":5411541776,"va":5387844450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23697648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844272,"raw_hex":"e8f0986901","size":5,"soft_fail":false,"target_va":5411542064,"va":5387844411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23697868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844272,"raw_hex":"e8cc996901","size":5,"soft_fail":false,"target_va":5411542296,"va":5387844423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5961256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844272,"raw_hex":"e8d809a5ff","size":5,"soft_fail":false,"target_va":5381883088,"va":5387844339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":10244,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":10216,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":9908,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":25767,"source_line":845,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":9776,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":70648,"source_line":1292,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":9964,"parent_record_offset":9908,"ancestor_record_offset":9612,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":9612,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":9868,"parent_record_offset":9848,"ancestor_record_offset":9612,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":9612,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":9612,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":9992,"parent_record_offset":9964,"ancestor_record_offset":9612,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":9612,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":9612,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":10060,"parent_record_offset":9908,"ancestor_record_offset":9612,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1461,"record_offset":10184,"parent_record_offset":9908,"ancestor_record_offset":9612,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::below_to_above
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":10284,"rva":19135344,"va":5387844464}
// 编译实例 RVAs: 0x123fb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13488071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844464,"raw_hex":"e8393032ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5387844594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23697380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844464,"raw_hex":"e8e4976901","size":5,"soft_fail":false,"target_va":5411542064,"va":5387844679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23697600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844464,"raw_hex":"e8c0986901","size":5,"soft_fail":false,"target_va":5411542296,"va":5387844691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5961475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387844464,"raw_hex":"e8fd08a5ff","size":5,"soft_fail":false,"target_va":5381883088,"va":5387844558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":10892,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":10584,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":25767,"source_line":845,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":10444,"parent_record_offset":10284,"ancestor_record_offset":10284,"inlinee":70648,"source_line":1292,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":10960,"parent_record_offset":10928,"ancestor_record_offset":10284,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":10284,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":10284,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":10928,"parent_record_offset":10892,"ancestor_record_offset":10284,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":10640,"parent_record_offset":10584,"ancestor_record_offset":10284,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":10520,"parent_record_offset":10496,"ancestor_record_offset":10284,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":10284,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":10284,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1461,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":10284,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":11048,"parent_record_offset":10892,"ancestor_record_offset":10284,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":10668,"parent_record_offset":10640,"ancestor_record_offset":10284,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":10696,"parent_record_offset":10668,"ancestor_record_offset":10284,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":10284,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":10736,"parent_record_offset":10584,"ancestor_record_offset":10284,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1461,"record_offset":10860,"parent_record_offset":10584,"ancestor_record_offset":10284,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/01461.jsonl.gz"}
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
// 完整结构及原始类型记录: trim_rows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::trim_rows::RowTrimmerSink
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>", "constructor": "utils::geometry::split_heightmap::split_heightmap_by_threshold", "arguments": ["system_wall_constructor::utils::trim_rows::RowTrimmerSink"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::trim_rows::TrimmedRow
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::utils::trim_rows::TrimmedRow,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::utils::trim_rows::TrimmedRow", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: trim_rows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: trim_rows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::above
// 信息 ID: f5fa317d9e1c10a5952867a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>; module=3034, inline_record=484; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>; module=3034, inline_record=1088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::above_to_below
// 信息 ID: 1e3c8a21d4b722a13e1d9bce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink> @ 0x1fe08b9 -> 0x123fab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::below_to_above
// 信息 ID: fe8904063e1229593b12789a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink> @ 0x1fe08fe -> 0x123fb70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::trim_rows
// 信息 ID: b4e592d186d0f4e44342e211
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0x769db6 -> 0x123eef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::trim_rows::closure$0
// 信息 ID: 19e84b4d668cdfc1d8ec77b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::trim_rows::trim_rows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::trim_rows::trim_rows; module=1461, inline_record=5748; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::trim_rows::trim_rows; module=1461, inline_record=5860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
