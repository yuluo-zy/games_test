// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/toolbar_button.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 104061a7e1ebbc75d2f827ac2551d5ad70134e146b4f4db9759da1977fbf8796
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_button.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3910
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::toolbar_button_keep_focus
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3910,"record_offset":1644,"rva":36967600,"va":5405676720}
// 编译实例 RVAs: 0x23414b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5862139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e8fb725900","size":5,"soft_fail":false,"target_va":5411541584,"va":5405679440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e88afbffff","size":5,"soft_fail":false,"target_va":5405675648,"va":5405676785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5866493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e8fd835900","size":5,"soft_fail":false,"target_va":5411543360,"va":5405676862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24291752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e858568dfe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405677283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24286566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e89a6a8dfe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405677201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24292630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e8ea528dfe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405679105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24292060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e824558dfe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405676759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5915453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e83d435a00","size":5,"soft_fail":false,"target_va":5411594832,"va":5405679374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5857671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405676720,"raw_hex":"e887615900","size":5,"soft_fail":false,"target_va":5411537104,"va":5405679428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":4612,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":8080,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3910,"record_offset":7516,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":4676,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":4208,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3910,"record_offset":7484,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3910,"record_offset":17256,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":3240,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":1816,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":2264,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":1644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":1644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":1644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":1644,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":4396,"parent_record_offset":4364,"ancestor_record_offset":1644,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":1644,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":1644,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":1644,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":7224,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":6160,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":6244,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":7420,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":6076,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":1644,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3910,"record_offset":7596,"parent_record_offset":7548,"ancestor_record_offset":1644,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":7628,"parent_record_offset":7596,"ancestor_record_offset":1644,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":8116,"parent_record_offset":8080,"ancestor_record_offset":1644,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":1644,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":1644,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":1644,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":1644,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":1644,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":5864,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":4316,"parent_record_offset":4232,"ancestor_record_offset":1644,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":1644,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":4708,"parent_record_offset":4676,"ancestor_record_offset":1644,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":6336,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":5032,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3910,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":1644,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":1644,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":1644,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":1644,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":5088,"parent_record_offset":5056,"ancestor_record_offset":1644,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":4948,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":1644,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":4864,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":5460,"parent_record_offset":5056,"ancestor_record_offset":1644,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":5404,"parent_record_offset":5056,"ancestor_record_offset":1644,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3910,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":1644,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":1644,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":1644,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":4260,"parent_record_offset":4232,"ancestor_record_offset":1644,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":1644,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3910,"record_offset":7548,"parent_record_offset":7516,"ancestor_record_offset":1644,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3910,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":1644,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":1644,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":1644,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":1644,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":1644,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":1644,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3704,"parent_record_offset":3532,"ancestor_record_offset":1644,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3910,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":1644,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":8156,"parent_record_offset":8116,"ancestor_record_offset":1644,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3910,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":1644,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":1644,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":7672,"parent_record_offset":7628,"ancestor_record_offset":1644,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3910,"record_offset":1964,"parent_record_offset":1932,"ancestor_record_offset":1644,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3910,"record_offset":3376,"parent_record_offset":3336,"ancestor_record_offset":1644,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":1644,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":1644,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":5248,"parent_record_offset":5112,"ancestor_record_offset":1644,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":8244,"parent_record_offset":8116,"ancestor_record_offset":1644,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3910,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":1644,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3910,"record_offset":4164,"parent_record_offset":3336,"ancestor_record_offset":1644,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3910,"record_offset":3768,"parent_record_offset":3480,"ancestor_record_offset":1644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3910,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":1644,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":1644,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":1644,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":1644,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3910,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":1644,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":1644,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":1644,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":1644,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":5328,"parent_record_offset":5296,"ancestor_record_offset":1644,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":1644,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":1644,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3910,"record_offset":4536,"parent_record_offset":4364,"ancestor_record_offset":1644,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":1644,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":1644,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":2932,"parent_record_offset":2904,"ancestor_record_offset":1644,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":4792,"parent_record_offset":4708,"ancestor_record_offset":1644,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":2580,"parent_record_offset":2464,"ancestor_record_offset":1644,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3910,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":1644,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3910,"record_offset":2292,"parent_record_offset":2264,"ancestor_record_offset":1644,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3910,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":1644,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3910,"record_offset":2400,"parent_record_offset":2364,"ancestor_record_offset":1644,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3910,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":1644,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3910,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":1644,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3910,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":1644,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3910,"record_offset":2904,"parent_record_offset":2364,"ancestor_record_offset":1644,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":1644,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1644,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":2064,"parent_record_offset":2032,"ancestor_record_offset":1644,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":2160,"parent_record_offset":2064,"ancestor_record_offset":1644,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":1644,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":1848,"parent_record_offset":1816,"ancestor_record_offset":1644,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3910,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":1644,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3910,"record_offset":2464,"parent_record_offset":2432,"ancestor_record_offset":1644,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3910,"record_offset":2748,"parent_record_offset":2464,"ancestor_record_offset":1644,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3910,"record_offset":2720,"parent_record_offset":2464,"ancestor_record_offset":1644,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3910,"record_offset":2524,"parent_record_offset":2464,"ancestor_record_offset":1644,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3910,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":1644,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":3272,"parent_record_offset":3240,"ancestor_record_offset":1644,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":17320,"parent_record_offset":17288,"ancestor_record_offset":1644,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":17820,"parent_record_offset":17560,"ancestor_record_offset":1644,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3910,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":1644,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":3480,"parent_record_offset":3336,"ancestor_record_offset":1644,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3910,"record_offset":3740,"parent_record_offset":3480,"ancestor_record_offset":1644,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3910,"record_offset":3432,"parent_record_offset":3404,"ancestor_record_offset":1644,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3910,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":1644,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3910,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":1644,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_app_mode_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3910,"record_offset":18300,"rva":36970464,"va":5405679584}
// 编译实例 RVAs: 0x2341fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31323255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405679584,"raw_hex":"e8890b22fe","size":5,"soft_fail":false,"target_va":5374356912,"va":5405680162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5648652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405679584,"raw_hex":"e80c315600","size":5,"soft_fail":false,"target_va":5411330256,"va":5405681599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":26696,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":22136,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":19516,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3910,"record_offset":18468,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3910,"record_offset":19164,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":22644,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":26904,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":19856,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":22364,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mode_manager::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":3910,"record_offset":26544,"parent_record_offset":18300,"ancestor_record_offset":18300,"inlinee":31686,"source_line":54,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":19356,"parent_record_offset":19324,"ancestor_record_offset":18300,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":27044,"parent_record_offset":27016,"ancestor_record_offset":18300,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":27128,"parent_record_offset":27104,"ancestor_record_offset":18300,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":27104,"parent_record_offset":27072,"ancestor_record_offset":18300,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":19404,"parent_record_offset":19380,"ancestor_record_offset":18300,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":19380,"parent_record_offset":19356,"ancestor_record_offset":18300,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":27072,"parent_record_offset":27044,"ancestor_record_offset":18300,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":19324,"parent_record_offset":19280,"ancestor_record_offset":18300,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":18300,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":18300,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":22948,"parent_record_offset":22924,"ancestor_record_offset":18300,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":22972,"parent_record_offset":22948,"ancestor_record_offset":18300,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":19280,"parent_record_offset":19236,"ancestor_record_offset":18300,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":27016,"parent_record_offset":26988,"ancestor_record_offset":18300,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":18784,"parent_record_offset":18556,"ancestor_record_offset":18300,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":25856,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":23168,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":24264,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":26136,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":23084,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":20256,"parent_record_offset":19924,"ancestor_record_offset":18300,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":19480,"parent_record_offset":19192,"ancestor_record_offset":18300,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":26720,"parent_record_offset":26696,"ancestor_record_offset":18300,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3910,"record_offset":19924,"parent_record_offset":19880,"ancestor_record_offset":18300,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3910,"record_offset":19192,"parent_record_offset":19164,"ancestor_record_offset":18300,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3910,"record_offset":19236,"parent_record_offset":19192,"ancestor_record_offset":18300,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3910,"record_offset":18532,"parent_record_offset":18492,"ancestor_record_offset":18300,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":18556,"parent_record_offset":18532,"ancestor_record_offset":18300,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":22168,"parent_record_offset":22136,"ancestor_record_offset":18300,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":19540,"parent_record_offset":19516,"ancestor_record_offset":18300,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":21256,"parent_record_offset":21232,"ancestor_record_offset":18300,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":20752,"parent_record_offset":20728,"ancestor_record_offset":18300,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":20776,"parent_record_offset":20752,"ancestor_record_offset":18300,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":21232,"parent_record_offset":21208,"ancestor_record_offset":18300,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":18300,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":24912,"parent_record_offset":24356,"ancestor_record_offset":18300,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":23776,"parent_record_offset":23252,"ancestor_record_offset":18300,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":26188,"parent_record_offset":26164,"ancestor_record_offset":18300,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":26252,"parent_record_offset":26224,"ancestor_record_offset":18300,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":21208,"parent_record_offset":19880,"ancestor_record_offset":18300,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":18300,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":18300,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":22668,"parent_record_offset":22644,"ancestor_record_offset":18300,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":20728,"parent_record_offset":19880,"ancestor_record_offset":18300,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":23252,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3910,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":18300,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":20876,"parent_record_offset":20852,"ancestor_record_offset":18300,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":20852,"parent_record_offset":20828,"ancestor_record_offset":18300,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":18300,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":25436,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":20828,"parent_record_offset":20800,"ancestor_record_offset":18300,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":20800,"parent_record_offset":20776,"ancestor_record_offset":18300,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":22924,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":25812,"parent_record_offset":25788,"ancestor_record_offset":18300,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":20468,"parent_record_offset":19880,"ancestor_record_offset":18300,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":24200,"parent_record_offset":23776,"ancestor_record_offset":18300,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":21128,"parent_record_offset":21104,"ancestor_record_offset":18300,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":18300,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3910,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":18300,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":26960,"parent_record_offset":26932,"ancestor_record_offset":18300,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":26988,"parent_record_offset":26960,"ancestor_record_offset":18300,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":26932,"parent_record_offset":26904,"ancestor_record_offset":18300,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":18300,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":18300,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3910,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":18300,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3910,"record_offset":21304,"parent_record_offset":21280,"ancestor_record_offset":18300,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3910,"record_offset":20316,"parent_record_offset":20288,"ancestor_record_offset":18300,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3910,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":18300,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3910,"record_offset":26580,"parent_record_offset":26544,"ancestor_record_offset":18300,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":19592,"parent_record_offset":19568,"ancestor_record_offset":18300,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":22200,"parent_record_offset":22168,"ancestor_record_offset":18300,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":19568,"parent_record_offset":19540,"ancestor_record_offset":18300,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":19672,"parent_record_offset":19648,"ancestor_record_offset":18300,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":18300,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":18600,"parent_record_offset":18556,"ancestor_record_offset":18300,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3910,"record_offset":20200,"parent_record_offset":19924,"ancestor_record_offset":18300,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3910,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":18300,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":18300,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":19616,"parent_record_offset":19592,"ancestor_record_offset":18300,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":20948,"parent_record_offset":20924,"ancestor_record_offset":18300,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":20988,"parent_record_offset":20852,"ancestor_record_offset":18300,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":22296,"parent_record_offset":22200,"ancestor_record_offset":18300,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":19648,"parent_record_offset":19568,"ancestor_record_offset":18300,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":22328,"parent_record_offset":22168,"ancestor_record_offset":18300,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3910,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":18300,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":18300,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":22804,"parent_record_offset":22768,"ancestor_record_offset":18300,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3910,"record_offset":19444,"parent_record_offset":19280,"ancestor_record_offset":18300,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":18656,"parent_record_offset":18556,"ancestor_record_offset":18300,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":25632,"parent_record_offset":22668,"ancestor_record_offset":18300,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":19008,"parent_record_offset":18556,"ancestor_record_offset":18300,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":20024,"parent_record_offset":19996,"ancestor_record_offset":18300,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":19996,"parent_record_offset":19956,"ancestor_record_offset":18300,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":24068,"parent_record_offset":24036,"ancestor_record_offset":18300,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":18712,"parent_record_offset":18684,"ancestor_record_offset":18300,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":18684,"parent_record_offset":18656,"ancestor_record_offset":18300,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3910,"record_offset":19032,"parent_record_offset":19008,"ancestor_record_offset":18300,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":18300,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":19724,"parent_record_offset":19672,"ancestor_record_offset":18300,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":18300,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$23::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3910,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":18300,"inlinee":24851,"source_line":21,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::mode_manager::impl$26::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":3910,"record_offset":26604,"parent_record_offset":26580,"ancestor_record_offset":18300,"inlinee":29713,"source_line":42,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":20288,"parent_record_offset":19880,"ancestor_record_offset":18300,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":20364,"parent_record_offset":20340,"ancestor_record_offset":18300,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":20388,"parent_record_offset":20364,"ancestor_record_offset":18300,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":18300,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3910,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":18300,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_toolbar_category_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3910,"record_offset":27312,"rva":36972528,"va":5405681648}
// 编译实例 RVAs: 0x23427f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31327215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405681648,"raw_hex":"e811fc21fe","size":5,"soft_fail":false,"target_va":5374354992,"va":5405682202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5647346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405681648,"raw_hex":"e8f22b5600","size":5,"soft_fail":false,"target_va":5411330256,"va":5405682905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":31296,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":30796,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":28536,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3910,"record_offset":27488,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3910,"record_offset":28184,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":31360,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":35168,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":28876,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":30984,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$8::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":3910,"record_offset":31264,"parent_record_offset":27312,"ancestor_record_offset":27312,"inlinee":27186,"source_line":34,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":28376,"parent_record_offset":28344,"ancestor_record_offset":27312,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":35328,"parent_record_offset":35296,"ancestor_record_offset":27312,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":35428,"parent_record_offset":35400,"ancestor_record_offset":27312,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":35400,"parent_record_offset":35360,"ancestor_record_offset":27312,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":28424,"parent_record_offset":28400,"ancestor_record_offset":27312,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":28400,"parent_record_offset":28376,"ancestor_record_offset":27312,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":35360,"parent_record_offset":35328,"ancestor_record_offset":27312,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":28344,"parent_record_offset":28300,"ancestor_record_offset":27312,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":27312,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":27828,"parent_record_offset":27804,"ancestor_record_offset":27312,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":31656,"parent_record_offset":31632,"ancestor_record_offset":27312,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":31680,"parent_record_offset":31656,"ancestor_record_offset":27312,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":28300,"parent_record_offset":28256,"ancestor_record_offset":27312,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":35296,"parent_record_offset":35264,"ancestor_record_offset":27312,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":27804,"parent_record_offset":27576,"ancestor_record_offset":27312,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":34652,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":33360,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":33444,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":34932,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":33276,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":29272,"parent_record_offset":28940,"ancestor_record_offset":27312,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":28500,"parent_record_offset":28212,"ancestor_record_offset":27312,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":31324,"parent_record_offset":31296,"ancestor_record_offset":27312,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3910,"record_offset":28940,"parent_record_offset":28900,"ancestor_record_offset":27312,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3910,"record_offset":28212,"parent_record_offset":28184,"ancestor_record_offset":27312,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3910,"record_offset":28256,"parent_record_offset":28212,"ancestor_record_offset":27312,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3910,"record_offset":27552,"parent_record_offset":27512,"ancestor_record_offset":27312,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":27576,"parent_record_offset":27552,"ancestor_record_offset":27312,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":30828,"parent_record_offset":30796,"ancestor_record_offset":27312,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":28560,"parent_record_offset":28536,"ancestor_record_offset":27312,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":27312,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":29768,"parent_record_offset":29744,"ancestor_record_offset":27312,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":29792,"parent_record_offset":29768,"ancestor_record_offset":27312,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":32944,"parent_record_offset":32920,"ancestor_record_offset":27312,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":27312,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":34128,"parent_record_offset":33536,"ancestor_record_offset":27312,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":32236,"parent_record_offset":31716,"ancestor_record_offset":27312,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":33056,"parent_record_offset":32920,"ancestor_record_offset":27312,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":33104,"parent_record_offset":33080,"ancestor_record_offset":27312,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":32920,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":31032,"parent_record_offset":31008,"ancestor_record_offset":27312,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":31056,"parent_record_offset":31032,"ancestor_record_offset":27312,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":31392,"parent_record_offset":31360,"ancestor_record_offset":27312,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":29744,"parent_record_offset":28900,"ancestor_record_offset":27312,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":31716,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3910,"record_offset":30168,"parent_record_offset":30144,"ancestor_record_offset":27312,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":29892,"parent_record_offset":29868,"ancestor_record_offset":27312,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":29868,"parent_record_offset":29844,"ancestor_record_offset":27312,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":30028,"parent_record_offset":30004,"ancestor_record_offset":27312,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":32724,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":27312,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":29816,"parent_record_offset":29792,"ancestor_record_offset":27312,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":31632,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":27312,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":29484,"parent_record_offset":28900,"ancestor_record_offset":27312,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":32660,"parent_record_offset":32236,"ancestor_record_offset":27312,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":30144,"parent_record_offset":30120,"ancestor_record_offset":27312,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":30120,"parent_record_offset":30096,"ancestor_record_offset":27312,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3910,"record_offset":32684,"parent_record_offset":32660,"ancestor_record_offset":27312,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":35232,"parent_record_offset":35200,"ancestor_record_offset":27312,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":35264,"parent_record_offset":35232,"ancestor_record_offset":27312,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":35200,"parent_record_offset":35168,"ancestor_record_offset":27312,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":28900,"parent_record_offset":28876,"ancestor_record_offset":27312,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":31008,"parent_record_offset":30984,"ancestor_record_offset":27312,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3910,"record_offset":27512,"parent_record_offset":27488,"ancestor_record_offset":27312,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3910,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":27312,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3910,"record_offset":29332,"parent_record_offset":29304,"ancestor_record_offset":27312,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3910,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":27312,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":28612,"parent_record_offset":28588,"ancestor_record_offset":27312,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":30860,"parent_record_offset":30828,"ancestor_record_offset":27312,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":28588,"parent_record_offset":28560,"ancestor_record_offset":27312,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":28692,"parent_record_offset":28668,"ancestor_record_offset":27312,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":27760,"parent_record_offset":27732,"ancestor_record_offset":27312,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":27620,"parent_record_offset":27576,"ancestor_record_offset":27312,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3910,"record_offset":29216,"parent_record_offset":28940,"ancestor_record_offset":27312,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3910,"record_offset":29940,"parent_record_offset":29916,"ancestor_record_offset":27312,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":29916,"parent_record_offset":29892,"ancestor_record_offset":27312,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":28636,"parent_record_offset":28612,"ancestor_record_offset":27312,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":29964,"parent_record_offset":29940,"ancestor_record_offset":27312,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":30004,"parent_record_offset":29868,"ancestor_record_offset":27312,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":28668,"parent_record_offset":28588,"ancestor_record_offset":27312,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":30948,"parent_record_offset":30828,"ancestor_record_offset":27312,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3910,"record_offset":28076,"parent_record_offset":28052,"ancestor_record_offset":27312,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":28816,"parent_record_offset":28792,"ancestor_record_offset":27312,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":31512,"parent_record_offset":31476,"ancestor_record_offset":27312,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3910,"record_offset":28464,"parent_record_offset":28300,"ancestor_record_offset":27312,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":27676,"parent_record_offset":27576,"ancestor_record_offset":27312,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":34848,"parent_record_offset":31392,"ancestor_record_offset":27312,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":28028,"parent_record_offset":27576,"ancestor_record_offset":27312,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":29040,"parent_record_offset":29012,"ancestor_record_offset":27312,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":29012,"parent_record_offset":28972,"ancestor_record_offset":27312,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":32528,"parent_record_offset":32496,"ancestor_record_offset":27312,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":27732,"parent_record_offset":27704,"ancestor_record_offset":27312,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":27704,"parent_record_offset":27676,"ancestor_record_offset":27312,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3910,"record_offset":28052,"parent_record_offset":28028,"ancestor_record_offset":27312,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":27312,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":28744,"parent_record_offset":28692,"ancestor_record_offset":27312,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":27312,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":29304,"parent_record_offset":28900,"ancestor_record_offset":27312,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":29380,"parent_record_offset":29356,"ancestor_record_offset":27312,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":29404,"parent_record_offset":29380,"ancestor_record_offset":27312,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":29356,"parent_record_offset":29332,"ancestor_record_offset":27312,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3910,"record_offset":30052,"parent_record_offset":30028,"ancestor_record_offset":27312,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::tod_v2_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3910,"record_offset":35520,"rva":36974384,"va":5405683504}
// 编译实例 RVAs: 0x2342f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5646123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405683504,"raw_hex":"e82b275600","size":5,"soft_fail":false,"target_va":5411330256,"va":5405684128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":37320,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":41580,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":38764,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":36372,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3910,"record_offset":35676,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":39452,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":37376,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":36712,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":39108,"parent_record_offset":35520,"ancestor_record_offset":35520,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":36948,"parent_record_offset":36924,"ancestor_record_offset":35520,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":37516,"parent_record_offset":37488,"ancestor_record_offset":35520,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":37600,"parent_record_offset":37576,"ancestor_record_offset":35520,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":37576,"parent_record_offset":37544,"ancestor_record_offset":35520,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":36996,"parent_record_offset":36972,"ancestor_record_offset":35520,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":36972,"parent_record_offset":36948,"ancestor_record_offset":35520,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3910,"record_offset":37544,"parent_record_offset":37516,"ancestor_record_offset":35520,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":36924,"parent_record_offset":36900,"ancestor_record_offset":35520,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":35520,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":40392,"parent_record_offset":40368,"ancestor_record_offset":35520,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":40416,"parent_record_offset":40392,"ancestor_record_offset":35520,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":37488,"parent_record_offset":37460,"ancestor_record_offset":35520,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3910,"record_offset":35992,"parent_record_offset":35764,"ancestor_record_offset":35520,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":41076,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":41488,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":40452,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":41272,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":41404,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3910,"record_offset":37100,"parent_record_offset":36768,"ancestor_record_offset":35520,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":37344,"parent_record_offset":37320,"ancestor_record_offset":35520,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3910,"record_offset":41608,"parent_record_offset":41580,"ancestor_record_offset":35520,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3910,"record_offset":36768,"parent_record_offset":36736,"ancestor_record_offset":35520,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3910,"record_offset":35740,"parent_record_offset":35700,"ancestor_record_offset":35520,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":35764,"parent_record_offset":35740,"ancestor_record_offset":35520,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":38812,"parent_record_offset":38764,"ancestor_record_offset":35520,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3910,"record_offset":36396,"parent_record_offset":36372,"ancestor_record_offset":35520,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":40120,"parent_record_offset":40096,"ancestor_record_offset":35520,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":40568,"parent_record_offset":40544,"ancestor_record_offset":35520,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":39596,"parent_record_offset":39572,"ancestor_record_offset":35520,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":40096,"parent_record_offset":40072,"ancestor_record_offset":35520,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":40144,"parent_record_offset":40120,"ancestor_record_offset":35520,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3910,"record_offset":40072,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":39156,"parent_record_offset":39132,"ancestor_record_offset":35520,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":39180,"parent_record_offset":39156,"ancestor_record_offset":35520,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3910,"record_offset":39484,"parent_record_offset":39452,"ancestor_record_offset":35520,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":40544,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3910,"record_offset":39572,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3910,"record_offset":39976,"parent_record_offset":39952,"ancestor_record_offset":35520,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":39684,"parent_record_offset":39656,"ancestor_record_offset":35520,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":39656,"parent_record_offset":39628,"ancestor_record_offset":35520,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3910,"record_offset":39820,"parent_record_offset":39796,"ancestor_record_offset":35520,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":39628,"parent_record_offset":39596,"ancestor_record_offset":35520,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":40368,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":40168,"parent_record_offset":40144,"ancestor_record_offset":35520,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3910,"record_offset":40284,"parent_record_offset":39484,"ancestor_record_offset":35520,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":40008,"parent_record_offset":39596,"ancestor_record_offset":35520,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3910,"record_offset":39952,"parent_record_offset":39596,"ancestor_record_offset":35520,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3910,"record_offset":40032,"parent_record_offset":40008,"ancestor_record_offset":35520,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":37432,"parent_record_offset":37404,"ancestor_record_offset":35520,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":37460,"parent_record_offset":37432,"ancestor_record_offset":35520,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3910,"record_offset":37404,"parent_record_offset":37376,"ancestor_record_offset":35520,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":36736,"parent_record_offset":36712,"ancestor_record_offset":35520,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3910,"record_offset":39132,"parent_record_offset":39108,"ancestor_record_offset":35520,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3910,"record_offset":35700,"parent_record_offset":35676,"ancestor_record_offset":35520,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3910,"record_offset":37160,"parent_record_offset":37132,"ancestor_record_offset":35520,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3910,"record_offset":37068,"parent_record_offset":37044,"ancestor_record_offset":35520,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":36448,"parent_record_offset":36424,"ancestor_record_offset":35520,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":38860,"parent_record_offset":38812,"ancestor_record_offset":35520,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":36424,"parent_record_offset":36396,"ancestor_record_offset":35520,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3910,"record_offset":36528,"parent_record_offset":36504,"ancestor_record_offset":35520,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":35948,"parent_record_offset":35920,"ancestor_record_offset":35520,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3910,"record_offset":35808,"parent_record_offset":35764,"ancestor_record_offset":35520,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3910,"record_offset":37044,"parent_record_offset":36768,"ancestor_record_offset":35520,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3910,"record_offset":39732,"parent_record_offset":39708,"ancestor_record_offset":35520,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":38908,"parent_record_offset":38860,"ancestor_record_offset":35520,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":36472,"parent_record_offset":36448,"ancestor_record_offset":35520,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":39756,"parent_record_offset":39732,"ancestor_record_offset":35520,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":39796,"parent_record_offset":39656,"ancestor_record_offset":35520,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":38980,"parent_record_offset":38860,"ancestor_record_offset":35520,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":36504,"parent_record_offset":36424,"ancestor_record_offset":35520,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3910,"record_offset":39012,"parent_record_offset":38812,"ancestor_record_offset":35520,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3910,"record_offset":36264,"parent_record_offset":36240,"ancestor_record_offset":35520,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":36652,"parent_record_offset":36628,"ancestor_record_offset":35520,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3910,"record_offset":40248,"parent_record_offset":40212,"ancestor_record_offset":35520,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":35864,"parent_record_offset":35764,"ancestor_record_offset":35520,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3910,"record_offset":36216,"parent_record_offset":35764,"ancestor_record_offset":35520,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":36868,"parent_record_offset":36840,"ancestor_record_offset":35520,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":36840,"parent_record_offset":36800,"ancestor_record_offset":35520,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":39876,"parent_record_offset":39844,"ancestor_record_offset":35520,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":35920,"parent_record_offset":35892,"ancestor_record_offset":35520,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3910,"record_offset":35892,"parent_record_offset":35864,"ancestor_record_offset":35520,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3910,"record_offset":36240,"parent_record_offset":36216,"ancestor_record_offset":35520,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":36628,"parent_record_offset":36604,"ancestor_record_offset":35520,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":36580,"parent_record_offset":36528,"ancestor_record_offset":35520,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3910,"record_offset":36604,"parent_record_offset":36580,"ancestor_record_offset":35520,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":37132,"parent_record_offset":36736,"ancestor_record_offset":35520,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":37208,"parent_record_offset":37184,"ancestor_record_offset":35520,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":37232,"parent_record_offset":37208,"ancestor_record_offset":35520,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3910,"record_offset":37184,"parent_record_offset":37160,"ancestor_record_offset":35520,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03910.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3910,"record_offset":39844,"parent_record_offset":39820,"ancestor_record_offset":35520,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03910.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_button.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 36
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::reader::EventReader", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::registry::EventRegistry::register_event<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::registry::EventRegistry::register_event", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> >", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)", "void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> > >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> > > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)> >", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> > >", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> > >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> > > > >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> >", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> > >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<country_core::components::Ui2dExtent> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::TaffyNode>,ref$<country_core::components::UiNav> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::writer::EventWriter<toolbar::ui_interaction::UiClickEvent>)> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::collections::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::collections::Events", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$9::init_state<bevy_ecs::event::collections::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns> >", "constructor": "bevy_ecs::system::system_param::impl$9::init_state", "arguments": ["bevy_ecs::event::collections::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_button.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_button.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_app_mode_button
// 信息 ID: 3a71524aa6585cd666842aef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 59628e2d9a1564505f1a1f42: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > > @ 0x1e14cfd -> 0x2341fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x1eaac18 -> 0x2341fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_toolbar_category_button
// 信息 ID: 6f519678886c761be64133cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0aea34a098357f624f5c6f72: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> > > @ 0x1e0a69d -> 0x23427f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> > @ 0x1eaa5c8 -> 0x23427f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::tod_v2_button
// 信息 ID: 18e5bd6b1f26705b665e9f2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6e9e316cd7e29b7b39359fd8: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > > @ 0x1e19c13 -> 0x2342f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x1eaa19d -> 0x2342f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::toolbar_button_keep_focus
// 信息 ID: 7af32513f5a4b5bb9436e1a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 39f466558b55d203c3380920: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > > @ 0x1e14fe7 -> 0x23414b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > @ 0x1eaa89f -> 0x23414b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
